#![doc = r#"
An implementation of the Jordan model for iron losses in the core lamination.

The Jordan loss model for iron losses offers a simple calculation heuristic for
a sinusoidal flux density change over time. It separates iron losses into static
hysteresis losses and dynamic eddy current losses via the following formula:

`p = kh * f * B² + kec * (f * B)²`,

where `f` is the frequency and `B` is the amplitude of the flux density. The
hysteresis loss factor `kh` and the eddy current loss factor `kec` are derived
by fitting measured loss curves. See [[1]] and [[2]] for more.

This module offers the [`JordanModel`] struct, a simple container for the two
loss coefficients which provides the formula given above via its
[`JordanModel::losses`] method. The struct implements [`IsQuantityFunction`] and
can therefore be used as the [iron loss model](crate::Material::iron_losses) of
a [`Material`](crate::Material).

The coefficients can be obtained from measured loss curves by constructing an
[`IronLossData`] instance out of them and then fallibly converting it via
[`TryFrom`] into a [`JordanModel`]. Under the hood, the curves are fitted to the
loss equation using a least-square optimization with the coefficients being the
variables. The [`FailedCoefficientCalculation`] error type is returned in case
the fitting failed for some reason. Lastly, the types
[`IronLossCharacteristic`] and [`FluxDensityLossPair`] are used within the
construction of [`IronLossData`] to guard against bad input data on the type
level.

# Example

The image below shows a comparison between raw loss data and the fitted
[`JordanModel`] from `examples/jordan_model.rs`. While the model can represent
the loss behaviour at lower frequencies very well, it fails at higher
frequencies for this particular set of data points.

"#]
#![cfg_attr(
    docsrs,
    doc = "\n\n![](https://raw.githubusercontent.com/StefanMathis/stem_material/refs/heads/main/docs/ferromagnetic_characteristic_mod.svg \"Ferromagnetic characteristic\")"
)]
#![cfg_attr(
    not(docsrs),
    doc = "\n\n![>> Example image missing, copy folder docs from crate root to doc root folder (where index.html is) to display the image <<](../docs/ferromagnetic_characteristic_mod.svg)"
)]
#![doc = r#"

# Literature

> [[1]] Krings, A. and Soulard, J.: Overview and comparison of iron loss models
for electrical machines. EVRE Monaco, March 2010. URL:
https://www.researchgate.net/profile/Andreas-Krings/publication/228490936_Overview_and_Comparison_of_Iron_Loss_Models_for_Electrical_Machines/links/02e7e51935e2728dda000000/Overview-and-Comparison-of-Iron-Loss-Models-for-Electrical-Machines.pdf

> [[2]] Graham, C. D.: Physical origin of losses in conducting ferromagnetic
materials. Journal of Applied Physics, vol. 53, no. 11, pp. 8276-8280, Nov.1982
"#]

use argmin::{
    core::{CostFunction, State},
    solver::neldermead::NelderMead,
};
use dyn_quantity::DynQuantity;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "serde")]
use dyn_quantity::deserialize_quantity;

use uom::si::{
    f64::*, frequency::hertz, magnetic_flux_density::tesla, ratio::ratio,
    specific_power::watt_per_kilogram,
};
use var_quantity::IsQuantityFunction;

/**
Implementation of the Jordan iron loss model.

As discussed in the [crate-level documentation](crate::jordan_model), this
struct contains the hysteresis and eddy current loss coefficients of the Jordan
iron loss model:

`p = kh * f * B² + kec * (f * B)²`.

This model is valid for a magnetic flux density which changes sinusoidally over
time with the frequency `f` (normalized to 50 Hz) and the amplitude `B`
(normalized to 1.5 T). The [`losses`](JordanModel::losses) method uses this very
formula, dividing input flux density by 1.5 (see
[`JordanModel::reference_flux_density`]) and frequency by 50 (see
[`JordanModel::reference_frequency`]).
These normalization factors correspond to those usually used in literature, see
e.g. eq. (6.4.10) and (6.4.11) in [[1]].

# Constructing a Jordan loss model

If the coefficients are known, a [`JordanModel`] can be constructed via the
default field assignment constructor (the
[One True Constructor](https://doc.rust-lang.org/nomicon/constructors.html)).
Alternatively, the coefficients can be derived by fitting loss curves into the
loss equation. This is done by first creating an [`IronLossData`] struct and
then fallibly converting it into a [`JordanModel`] using [`TryFrom`]. Under the
hood, a least-square minimization / optimization is performed during conversion
to find the coefficients which match the given curves the best. See
[`IronLossData`] for more.

# Usage in `Material`

This struct is meant to be used for the
[`Material::iron_losses`](crate::Material::iron_losses), hence it implements
[`IsQuantityFunction`]. Inside the [`IsQuantityFunction::call`] function, the
input conditions are searched for an entry whose unit corresponds to that of
the magnetic flux density and another one which matches that of the frequency.
If either one cannot be found, a value of zero is assumed, which means that the
returned losses are zero as well:

```
use stem_material::*;
use uom::si::specific_power::watt_per_kilogram;
use uom::si::thermodynamic_temperature::degree_celsius;
use uom::si::frequency::hertz;
use uom::si::magnetic_flux_density::tesla;
use var_quantity::*;

let model = JordanModel {
    hysteresis_coefficient: SpecificPower::new::<watt_per_kilogram>(1.0),
    eddy_current_coefficient: SpecificPower::new::<watt_per_kilogram>(0.5),
};

let conditions = &[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()];
assert_eq!(model.call(conditions).value, 0.0);

// This call returns the sum of the coefficients, because the input matches
// the reference values and therefore the resulting `f` and `B` are 1
let conditions = &[MagneticFluxDensity::new::<tesla>(1.5).into(), Frequency::new::<hertz>(50.0).into()];
assert_eq!(model.call(conditions).value, 1.5);
```

# Serialization and deserialization

A [`JordanModel`] is serialized as one would expect: A struct with two fields.
However, it can be deserialized both from said two-fields representation and
from that of [`IronLossData`]. In case of the latter, the serialized data is
first deserialized into [`IronLossData`], which is then converted into a
[`JordanModel`]. Since an untagged enum is used for deserialization, it is not
necessary to use a tag in the latter case. This is shown below using the
yaml-format:

```ignore
JordanModel:
  hysteresis_coefficient: 1 W/kg
  eddy_current_coefficient: 1 W/kg
```

or

```ignore
JordanModel:
  - frequency: 50.0 Hz
    characteristic:
    - flux_density: 0.5 T
      specific_loss: 0.86 W/kg
    - flux_density: 0.6 T
      specific_loss: 1.16 W/kg
... (more entries for the loss curves)
```
both result in a [`JordanModel`], provided the conversion doesn't fail in case
of the latter.

# Literature

> [[1]] Müller, G., Vogt, K. and Ponick, B.: Berechnung elektrischer Maschinen,
6th edition, Wiley-VCH, 2008.
 */
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "serde_impl::JordanModelDeEnum"))]
pub struct JordanModel {
    /// Static hysteresis loss coefficient `kh`.
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
    pub hysteresis_coefficient: SpecificPower,
    /// Dynamic eddy current loss coefficient `kec`.
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
    pub eddy_current_coefficient: SpecificPower,
}

impl JordanModel {
    /**
    Creates a new [`JordanModel`] from its coefficients.
     */
    pub fn new(
        hysteresis_coefficient: SpecificPower,
        eddy_current_coefficient: SpecificPower,
    ) -> Self {
        return Self {
            hysteresis_coefficient,
            eddy_current_coefficient,
        };
    }

    /**
    Returns the "reference frequency" of 50 Hz used in the model.

    A frequency input to [`JordanModel::losses`] or [`JordanModel::call`] is
    divided by this value before being inserted into the model equation.

    # Examples

    ```
    use stem_material::JordanModel;
    use uom::si::frequency::hertz;

    assert_eq!(JordanModel::reference_frequency().get::<hertz>(), 50.0);
    ```
     */
    pub fn reference_frequency() -> Frequency {
        return Frequency::new::<hertz>(50.0);
    }

    /**
    Returns the "reference flux density" of 1.5 T used in the model.

    A flux density input to [`JordanModel::losses`] or [`JordanModel::call`] is
    divided by this value before being inserted into the model equation.

    # Examples

    ```
    use stem_material::JordanModel;
    use uom::si::magnetic_flux_density::tesla;

    assert_eq!(JordanModel::reference_flux_density().get::<tesla>(), 1.50);
    ```
     */
    pub fn reference_flux_density() -> MagneticFluxDensity {
        return MagneticFluxDensity::new::<tesla>(1.5);
    }

    /**
    Returns the specific losses for a sinusoidal changing magnetic flux density
    with the amplitude `magnetic_flux_density` and the specified `frequency`.

    This function returns the result ``p of the equation:

    `p = kh * f * B² + kec * (f * B)²`,

    where `kh` corresponds to [`JordanModel::hysteresis_coefficient`] and `kec`
    corresponds to [`JordanModel::eddy_current_coefficient`]. The arguments
    are normalized using [`JordanModel::reference_frequency`] and
    [`JordanModel::reference_flux_density`]:

    `B = magnetic_flux_density / JordanModel::reference_flux_density()`

    `f = frequency / JordanModel::reference_frequency()`

    The [`IsQuantityFunction::call`] implementation for [`JordanModel`] uses
    this function after identifying `magnetic_flux_density` and `frequency` from
    the `conditions`.

    # Examples

    ```
    use stem_material::*;
    use uom::si::specific_power::watt_per_kilogram;
    use uom::si::thermodynamic_temperature::degree_celsius;
    use uom::si::frequency::hertz;
    use uom::si::magnetic_flux_density::tesla;

    let model = JordanModel {
        hysteresis_coefficient: SpecificPower::new::<watt_per_kilogram>(1.0),
        eddy_current_coefficient: SpecificPower::new::<watt_per_kilogram>(0.5),
    };

    // This call returns the sum of the coefficients, because the input matches
    // the reference values and therefore the resulting `f` and `B` are 1
    assert_eq!(model.losses(MagneticFluxDensity::new::<tesla>(1.5), Frequency::new::<hertz>(50.0)).value, 1.5);

    // Double the frequency - Losses rise drastically (nonlinear dependency)
    assert_eq!(model.losses(MagneticFluxDensity::new::<tesla>(1.5), Frequency::new::<hertz>(100.0)).value, 5.0);
    ```
    */
    pub fn losses(
        &self,
        magnetic_flux_density: MagneticFluxDensity,
        frequency: Frequency,
    ) -> SpecificPower {
        return losses(
            magnetic_flux_density,
            frequency,
            self.eddy_current_coefficient,
            self.hysteresis_coefficient,
        );
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl IsQuantityFunction for JordanModel {
    fn call(&self, influencing_factors: &[DynQuantity<f64>]) -> DynQuantity<f64> {
        let mut flux_density = MagneticFluxDensity::new::<tesla>(0.0);
        let mut frequency = Frequency::new::<hertz>(0.0);
        for factor in influencing_factors {
            if let Ok(fd) = MagneticFluxDensity::try_from(*factor) {
                flux_density = fd;
            } else if let Ok(f) = Frequency::try_from(*factor) {
                frequency = f;
            }
        }
        return self.losses(flux_density, frequency).into();
    }
}

/**
Actual loss calculation function. Factored out from the [`JordanModel`] method
of the same name because it is also used in [`TryFrom<IronLossData>`].s
 */
fn losses(
    flux_density: MagneticFluxDensity,
    frequency: Frequency,
    hysteresis_coefficient: SpecificPower,
    eddy_current_coefficient: SpecificPower,
) -> SpecificPower {
    let f_norm = JordanModel::reference_frequency();
    let b_norm = JordanModel::reference_flux_density();

    return hysteresis_coefficient
        * (frequency / f_norm)
        * (flux_density / b_norm).get::<ratio>().powi(2)
        + eddy_current_coefficient
            * (frequency / f_norm).get::<ratio>().powi(2)
            * (flux_density / b_norm).get::<ratio>().powi(2);
}

impl Default for JordanModel {
    fn default() -> Self {
        Self {
            hysteresis_coefficient: SpecificPower::new::<watt_per_kilogram>(0.0),
            eddy_current_coefficient: SpecificPower::new::<watt_per_kilogram>(0.0),
        }
    }
}

// =============================================================================

/**
This struct is a "flattened" version of [`IronLossData`]. It is not meant to be
used on its own and is just exposed so the optimization result of
[`IronLossData::solve_for_coefficients`] can be examined. See its docstring for
more.
 */
pub struct FitLossCurve {
    frequencies: Vec<Frequency>,
    flux_densities: Vec<MagneticFluxDensity>,
    specific_losses: Vec<SpecificPower>,
}

impl CostFunction for FitLossCurve {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let mut err = 0.0; // W/kg

        // Convert to SI units
        let hysteresis_coefficient = SpecificPower::new::<watt_per_kilogram>(p[0]);
        let eddy_current_coefficient = SpecificPower::new::<watt_per_kilogram>(p[1]);

        for (fi, (bi, pi)) in self
            .frequencies
            .iter()
            .zip(self.flux_densities.iter().zip(self.specific_losses.iter()))
        {
            err = err
                + (*pi - losses(*bi, *fi, hysteresis_coefficient, eddy_current_coefficient))
                    .get::<watt_per_kilogram>()
                    .powi(2);
        }
        Ok(err)
    }
}

/**
A container for multiple [`IronLossCharacteristic`]s.

This struct represents a full dataset of multiple loss characteristics at
different frequencies obtained from either a manufacturer data sheet or from own
measurements. Its main purpose is to be used for the calculation of the
[`JordanModel`] coefficients via the
[`solve_for_coefficients`](IronLossData::solve_for_coefficients) method. This
method returns the raw result of the underlying fitting as an
[`argmin::core::OptimizationResult`], which contains the coefficients. For
convenience, a [`TryFrom<IronLossData>`] implementation exists for
[`JordanModel`], which calls
[`solve_for_coefficients`](IronLossData::solve_for_coefficients) and then
unpacks the coefficients.
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IronLossData(pub Vec<IronLossCharacteristic>);

impl IronLossData {
    /**
    Performs least-square fitting of all the datapoints in `self` into the loss
    equation using the [`argmin`]. If the fitting succeeds, the raw
    [`argmin::core::OptimizationResult`] is returned, which can then be
    examined. In particular, the coefficients can be retrieved with the
    [`State::get_best_param`](`argmin::core::State::get_best_param`). As a
    convencience wrapper, a [`TryFrom<IronLossData>`] implementation exists for
    [`JordanModel`], which calls
    [`solve_for_coefficients`](IronLossData::solve_for_coefficients) and then
    unpacks the coefficients.

    # Examples

    ```
    use stem_material::*;
    use uom::si::specific_power::watt_per_kilogram;
    use uom::si::frequency::hertz;
    use uom::si::magnetic_flux_density::tesla;

    // Expose the get_best_param method
    use argmin::core::State;

    // First characteristic
    let frequency = Frequency::new::<hertz>(50.0);
    let mut datapoints = Vec::new();
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.5),
        SpecificPower::new::<watt_per_kilogram>(2.0)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.6),
        SpecificPower::new::<watt_per_kilogram>(2.5)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.7),
        SpecificPower::new::<watt_per_kilogram>(3.2)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.8),
        SpecificPower::new::<watt_per_kilogram>(4.0)
    ));
    let lc_50 = IronLossCharacteristic::new(frequency, datapoints);

    // Second characteristic
    let frequency = Frequency::new::<hertz>(100.0);
    let mut datapoints = Vec::new();
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.5),
        SpecificPower::new::<watt_per_kilogram>(5.0)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.6),
        SpecificPower::new::<watt_per_kilogram>(6.0)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.7),
        SpecificPower::new::<watt_per_kilogram>(8.0)
    ));
    datapoints.push(FluxDensityLossPair::new(
        MagneticFluxDensity::new::<tesla>(0.8),
        SpecificPower::new::<watt_per_kilogram>(12.0)
    ));
    let lc_100 = IronLossCharacteristic::new(frequency, datapoints);

    let iron_loss_data = IronLossData(vec![lc_50, lc_100]);
    let res = iron_loss_data.solve_for_coefficients().expect("fitting succeded");
    let c = res.state.get_best_param().expect("must contain coefficients");

    // First element is the hysteresis coefficient
    approx::assert_abs_diff_eq!(c[0], 9.528, epsilon=1e-3);

    // Second element is the eddy current coefficient
    approx::assert_abs_diff_eq!(c[1], 5.265, epsilon=1e-3);
    ```
     */
    pub fn solve_for_coefficients(
        &self,
    ) -> Result<
        argmin::core::OptimizationResult<
            FitLossCurve,
            NelderMead<Vec<f64>, f64>,
            argmin::core::IterState<Vec<f64>, (), (), (), (), f64>,
        >,
        FailedCoefficientCalculation,
    > {
        // Concatenate all vectors
        let mut num_elems: usize = 0;
        for characteristic in self.0.iter() {
            num_elems += characteristic.characteristic.len();
        }
        let mut frequencies_flat: Vec<Frequency> = Vec::with_capacity(num_elems);
        let mut flux_density_flat: Vec<MagneticFluxDensity> = Vec::with_capacity(num_elems);
        let mut specific_losses_flat: Vec<SpecificPower> = Vec::with_capacity(num_elems);

        for characteristic in self.0.iter() {
            let frequency = characteristic.frequency;

            for flux_density_and_specific_loss in characteristic.characteristic.iter().cloned() {
                frequencies_flat.push(frequency);
                flux_density_flat.push(flux_density_and_specific_loss.flux_density);
                specific_losses_flat.push(flux_density_and_specific_loss.specific_loss);
            }
        }

        let fit = FitLossCurve {
            frequencies: frequencies_flat,
            flux_densities: flux_density_flat,
            specific_losses: specific_losses_flat,
        };

        // All values in W/kg
        let start_values = vec![
            vec![3.0f64, 3.0f64],
            vec![2.0f64, 1.5f64],
            vec![1.0f64, 0.5f64],
        ];

        let solver = NelderMead::new(start_values)
            .with_sd_tolerance(0.0001)
            .map_err(|error| FailedCoefficientCalculation(Some(error)))?;

        // Run solver
        return argmin::core::Executor::new(fit, solver)
            .configure(|state| state.max_iters(200))
            .run()
            .map_err(|error| FailedCoefficientCalculation(Some(error)));
    }
}

impl TryFrom<IronLossData> for JordanModel {
    type Error = FailedCoefficientCalculation;
    fn try_from(value: IronLossData) -> Result<Self, Self::Error> {
        return (&value).try_into();
    }
}

impl TryFrom<&IronLossData> for JordanModel {
    type Error = FailedCoefficientCalculation;

    fn try_from(value: &IronLossData) -> Result<Self, Self::Error> {
        let res = value.solve_for_coefficients()?;
        let solution = res
            .state
            .get_best_param()
            .ok_or(FailedCoefficientCalculation(None))?;

        let hysteresis_coefficient = SpecificPower::new::<watt_per_kilogram>(solution[0]);
        let eddy_current_coefficient = SpecificPower::new::<watt_per_kilogram>(solution[1]);

        return Ok(JordanModel {
            hysteresis_coefficient,
            eddy_current_coefficient,
        });
    }
}

/**
A iron loss characteristic for a specific frequency.

This struct contains the iron loss characteristic (relationship between
sinusoidal magnetic flux density amplitude and losses) for a single frequency.
This characteristic is usually taken from the datasheet of the lamination
manufacturer or measured by applying a sinusoidal magnetic field at a given
frequency with different amplitudes to a sample. The losses within the sample
are then measured and form a [`FluxDensityLossPair`] datapoint together with the
corresponding amplitude.

One or more of these characteristics form an [`IronLossData`] dataset, which is
essentially just a vector of [`IronLossCharacteristic`]s. The dataset can then
be used to derive the coefficients of the [`JordanModel`].

# Examples

```
use stem_material::*;
use uom::si::specific_power::watt_per_kilogram;
use uom::si::frequency::hertz;
use uom::si::magnetic_flux_density::tesla;

// These datapoints might come from a manufacturer sheet.

// All datapoints were measured at this frequency
let frequency = Frequency::new::<hertz>(50.0);

// List of the individual datapoints as flux density - loss pairs.
let mut datapoints = Vec::new();
datapoints.push(FluxDensityLossPair::new(
    MagneticFluxDensity::new::<tesla>(0.5),
    SpecificPower::new::<watt_per_kilogram>(2.0)
));
datapoints.push(FluxDensityLossPair::new(
    MagneticFluxDensity::new::<tesla>(0.6),
    SpecificPower::new::<watt_per_kilogram>(2.5)
));
datapoints.push(FluxDensityLossPair::new(
    MagneticFluxDensity::new::<tesla>(0.7),
    SpecificPower::new::<watt_per_kilogram>(3.2)
));
datapoints.push(FluxDensityLossPair::new(
    MagneticFluxDensity::new::<tesla>(0.8),
    SpecificPower::new::<watt_per_kilogram>(4.0)
));
let loss_charactistic = IronLossCharacteristic::new(frequency, datapoints);
```
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IronLossCharacteristic {
    /// Frequency at which the charactistic has been measured. Should be a
    /// positive value (a negative frequency makes no sense from a physics point
    /// of view and at zero frequency the losses are also zero).
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
    pub frequency: Frequency,
    /// Collection of amplitude - losses datapoints. The order of these
    /// datapoints does not matter.
    pub characteristic: Vec<FluxDensityLossPair>,
}

impl IronLossCharacteristic {
    /**
    Creates a new [`IronLossCharacteristic`] from its fields.
     */
    pub fn new(frequency: Frequency, characteristic: Vec<FluxDensityLossPair>) -> Self {
        return Self {
            frequency,
            characteristic,
        };
    }

    /**
    Creates a new [`IronLossCharacteristic`] from its frequency, a slice of
    flux densities and one of specific losses. Each entry of the
    `flux_densities` is paired with the same-index entry of `specific_losses`
    to form a [`FluxDensityLossPair`]. If one slice is longer than the other,
    the surplus entries are discarded.
     */
    pub fn from_vecs(
        frequency: Frequency,
        flux_densities: &[MagneticFluxDensity],
        specific_losses: &[SpecificPower],
    ) -> Self {
        let mut characteristic = Vec::with_capacity(flux_densities.len());
        for (flux_density, specific_loss) in
            flux_densities.into_iter().zip(specific_losses.into_iter())
        {
            characteristic.push(FluxDensityLossPair::new(
                flux_density.clone(),
                specific_loss.clone(),
            ));
        }

        return Self::new(frequency, characteristic);
    }
}

/**
A single datapoint of an [`IronLossCharacteristic`].

This struct represents the specific losses in a lamination sheet created by a
sinusoidal magnetic field with the amplitude
[`FluxDensityLossPair::flux_density`] at a given frequency. It is meant to be
a building block of a [`IronLossCharacteristic`], where also the aforementioned
frequency is specified. See the docstring of [`IronLossCharacteristic`] for
examples.
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FluxDensityLossPair {
    /// Flux density of the datapoint.
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
    pub flux_density: MagneticFluxDensity,
    /// Specific losses of the datapoint.
    #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
    pub specific_loss: SpecificPower,
}

impl FluxDensityLossPair {
    /**
    Creates a new [`FluxDensityLossPair`] from its fields.
     */
    pub fn new(flux_density: MagneticFluxDensity, specific_loss: SpecificPower) -> Self {
        return Self {
            flux_density,
            specific_loss,
        };
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;

    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub(super) struct JordanModelAlias {
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
        hysteresis_coefficient: SpecificPower,
        #[cfg_attr(feature = "serde", serde(deserialize_with = "deserialize_quantity"))]
        eddy_current_coefficient: SpecificPower,
    }

    #[derive(DeserializeUntaggedVerboseError)]
    pub(super) enum JordanModelDeEnum {
        JordanModelAlias(JordanModelAlias),
        IronLossData(IronLossData),
    }

    impl TryFrom<JordanModelDeEnum> for JordanModel {
        type Error = FailedCoefficientCalculation;

        fn try_from(value: JordanModelDeEnum) -> Result<Self, Self::Error> {
            match value {
                JordanModelDeEnum::JordanModelAlias(alias) => Ok(JordanModel {
                    hysteresis_coefficient: alias.hysteresis_coefficient,
                    eddy_current_coefficient: alias.eddy_current_coefficient,
                }),
                JordanModelDeEnum::IronLossData(iron_loss_data) => iron_loss_data.try_into(),
            }
        }
    }
}

/**
A struct representing a failed [`JordanModel`] coefficient calculation attempt.

Calculating the coefficients of a [`JordanModel`] may fail due to a bad dataset.
The calculation uses a least-square minimization algorithm provided by the
[`argmin`] crate, which returns a [`argmin::core::Error`] when the calculation
fails. Even if no such error is created, the returned coefficient might still
be empty - this is represented by `FailedCoefficientCalculation(None)`.
 */
#[derive(Debug)]
pub struct FailedCoefficientCalculation(pub Option<argmin::core::Error>);

impl std::fmt::Display for FailedCoefficientCalculation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(cause) => {
                let original_message = cause.to_string();
                write!(
                    f,
                    "The calculation of the hysteresis loss coefficients failed,
                    likely due to bad input data. Original message: {original_message}."
                )
            }
            None => write!(
                f,
                "The calculation of the hysteresis loss coefficients failed,
                likely due to bad input data."
            ),
        }
    }
}

impl std::error::Error for FailedCoefficientCalculation {}
