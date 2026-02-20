#![doc = r#"
Implementation of a spline-based model for ferromagnetic permeability.

The strength of a magnetic field can be characterized by two different
quantities: The magnetic field strength `H` and the magnetic flux density `B`.
The former describes the magnetic potential of the field, while the latter
denotes the actual flux per area. They are related via the following equation:

`B = µ0 * µr * H`

with `µ0` being the [vacuum permeability](VACUUM_PERMEABILITY) and `µr` being
the relative permeability of the substance the field is passing through. For a
ferromagnetic material, `µr` itself is a function of `B` and therefore also of
`H`.

For real materials, this function cannot be expressed analytically and is
usually approximated by interpolating between measured datapoints. For physical
simulations which often involve iterative solvers, it is important that the
interpolation is "smooth", meaning that its derivatives do not "jump". For that
reason, spline interpolations are often used here.

This module offers the [`FerromagneticPermeability`] struct, which is
essentially a wrapper around two [`AkimaSpline`]s, one for `µr(H)` and one for
`µr(B)`. The struct is meant to constructed from measured datapoints provided
by the containers [`MagnetizationCurve`] and [`PolarizationCurve`]. The splines
are optimized for fast and stable numerical calculations when e.g. using an
iterative solver to determine the magnetization of an electrical motor. In
particular, this means the following:

1) The splines are strictly monotonously decreasing with higher `B` or `H`
values. For very low values of `B` or `H`, this leads to a small error. For
electric machines in particular, the saturation is usually high enough that this
error can be neglected (which is why e.g. commercial FEM software also modifies
the permability curves given by the user accordingly).
2) Datasheets or measurements usually have no datapoints for very high `B` or
`H` values (simply because these extrema don't usually occur in nature).
However, at the start of an iterative solver, extreme values may occur.
To guarantee a stable iteration, the aforementioned strictly monotonously
decreasing behaviour should still be guaranteed and in addition, no nonsensical
values (e.g. negative permeability) should be calculated. For this reasons, the
splines are extrapolated and clamped to guarantee these properties.

The image below shows these modifications:
"#]
#![cfg_attr(
    docsrs,
    doc = "\n\n![](https://raw.githubusercontent.com/StefanMathis/stem_material/refs/heads/main/docs/ferromagnetic_characteristic_mod.svg \"Ferromagnetic characteristic\")"
)]
#![cfg_attr(
    not(docsrs),
    doc = "\n\n![>> Example image missing, copy folder docs from crate root to doc root folder (where index.html is) to display the image <<](../docs/ferromagnetic_characteristic_mod.svg)"
)]

use std::f64::INFINITY;

use akima_spline::AkimaSpline;
use dyn_quantity::{DynQuantity, PredefUnit, Unit};
use uom::si::magnetic_field_strength::ampere_per_meter;
use uom::si::magnetic_flux_density::tesla;
use var_quantity::{IsQuantityFunction, QuantityFunction};

#[cfg(feature = "serde")]
use dyn_quantity::deserialize_vec_of_quantities;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{VACUUM_PERMEABILITY, VACUUM_PERMEABILITY_UNITLESS};

use uom::si::f64::*;

/**
A specialized variant of [`VarQuantity<f64>`](var_quantity::VarQuantity) for
relative permeability.

In principle, the [`FerromagneticPermeability`] case could be treated as a
[`IsQuantityFunction`] trait object, which would allow using [`VarQuantity<f64>`](var_quantity::VarQuantity) for the
[`Material::relative_permeability`](crate::material::Material::relative_permeability)
field. However, using the specialized enum variant
[`RelativePermeability::FerromagneticPermeability`] instead improves performance
drastically, since no dynamic dispatch is needed. Nevertheless, user-defined
permeability models are still supported via the
[`RelativePermeability::Function`] variant.
 */
#[derive(Clone, Debug)]
pub enum RelativePermeability {
    /**
    Optimization for the common case of a constant quantity. This avoids going
    through dynamic dispatch when accessing the value.
     */
    Constant(f64),
    /**
    Optimization for the common case of using the [`FerromagneticPermeability`]
    defined within this crate. This avoids going through dynamic dispatch when
    accessing the model.
     */
    FerromagneticPermeability(FerromagneticPermeability),
    /**
    Catch-all variant for any non-constant behaviour. Arbitrary behaviour
    can be realized with the contained [`IsQuantityFunction`] trait object, as
    long as the unit constraint outlined in the [`VarQuantity`] docstring is
    upheld.
     */
    Function(QuantityFunction<f64>),
}

#[cfg(feature = "serde")]
impl serde::Serialize for RelativePermeability {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        enum FerromagneticPermeabilityEnum<'a> {
            FerromagneticPermeability(&'a FerromagneticPermeability),
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum RelativePermeabilitySerde<'a> {
            Constant(f64),
            FerromagneticPermeabilityEnum(FerromagneticPermeabilityEnum<'a>),
            Function(&'a QuantityFunction<f64>),
        }

        let rp = match self {
            RelativePermeability::Constant(v) => RelativePermeabilitySerde::Constant(*v),
            RelativePermeability::FerromagneticPermeability(fp) => {
                RelativePermeabilitySerde::FerromagneticPermeabilityEnum(
                    FerromagneticPermeabilityEnum::FerromagneticPermeability(fp),
                )
            }
            RelativePermeability::Function(quantity_function) => {
                RelativePermeabilitySerde::Function(quantity_function)
            }
        };
        rp.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for RelativePermeability {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::Deserialize;

        /**
        This is a "fake" enum which just exists so the tag
        "FerromagneticPermeability" is deserialized correctly into [`RelativePermeability::FerromagneticPermeability`] instead of
        [`RelativePermeability::Function`].
         */
        #[derive(Deserialize)]
        enum FerromagneticPermeabilityEnum {
            FerromagneticPermeability(FerromagneticPermeability),
        }

        #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
        enum RelativePermeabilitySerde {
            Constant(f64),
            FerromagneticPermeabilityEnum(FerromagneticPermeabilityEnum),
            Function(QuantityFunction<f64>),
        }

        let rp_de = RelativePermeabilitySerde::deserialize(deserializer)?;
        let rp = match rp_de {
            RelativePermeabilitySerde::Constant(v) => RelativePermeability::Constant(v),
            RelativePermeabilitySerde::FerromagneticPermeabilityEnum(fp) => match fp {
                FerromagneticPermeabilityEnum::FerromagneticPermeability(jordan_model) => {
                    RelativePermeability::FerromagneticPermeability(jordan_model)
                }
            },
            RelativePermeabilitySerde::Function(quantity_function) => {
                RelativePermeability::Function(quantity_function)
            }
        };
        return Ok(rp);
    }
}

impl RelativePermeability {
    /**
    Matches against `self` and calculates the iron losses (or just return the
    value in case of the [`RelativePermeability::Constant`]) variant).
    */
    pub fn get(&self, influencing_factors: &[DynQuantity<f64>]) -> f64 {
        match self {
            Self::Constant(val) => val.clone(),
            Self::FerromagneticPermeability(model) => model.call(influencing_factors).try_into().expect("implementation of FerromagneticPermeability makes sure the returned value is always a f64"),
            Self::Function(fun) => fun.call(influencing_factors),
        }
    }

    /**
    Returns a reference to the underlying function if `self` is a
    [`RelativePermeability::Function`].
     */
    pub fn function(&self) -> Option<&dyn IsQuantityFunction> {
        match self {
            Self::Function(quantity_function) => return Some(quantity_function.as_ref()),
            _ => return None,
        }
    }
}

impl TryFrom<Box<dyn IsQuantityFunction>> for RelativePermeability {
    type Error = dyn_quantity::UnitsNotEqual;

    fn try_from(value: Box<dyn IsQuantityFunction>) -> Result<Self, Self::Error> {
        let wrapper = QuantityFunction::new(value)?;
        return Ok(Self::Function(wrapper));
    }
}

impl From<f64> for RelativePermeability {
    fn from(value: f64) -> Self {
        return Self::Constant(value);
    }
}

/**
A ferromagnetic permeability characteristic optimized for calculations.

Magnetic field strength `H` and flux density `B` are related via the following
equation:

`B = µ0 * µr * H`

with `µ0` being the [vacuum permeability](VACUUM_PERMEABILITY) and `µr` being
the relative permeability of a material. For a ferromagnetic material, `µr`
itself is a function of `B` and therefore also of `H`.

In this struct, the two functions `µr(B)` and `µr(H)` are represented via two
[`AkimaSpline`]s which are constructed from the datapoints of a
[`MagnetizationCurve`] or a [`PolarizationCurve`]. To optimize these splines for
numerical operations, they are modified according to the
[module-level documentation](crate::ferromagnetic_permeability).

# Constructing a relative permeability curve

For both [`MagnetizationCurve`] and [`PolarizationCurve`], there are dedicated
constructors
[`from_magnetization`](FerromagneticPermeability::from_magnetization) and
[`from_polarization`](FerromagneticPermeability::from_polarization) which
(fallibly) create the splines out of raw data. Additionally, it is also possible
to build the struct directly from its fields using manually created splines.

# Usage in `Material`

This struct is meant to be used for the
[`Material::relative_permeability`](crate::Material::relative_permeability),
hence it implements [`IsQuantityFunction`]. Inside the
[`IsQuantityFunction::call`] function, the input conditions are searched for an
entry whose unit corresponds to that of the magnetic field strength or flux
density. If one is found, the corresponding spline is selected and the resulting
relative permeability is returned. Otherwise, the relative permeability at 0 T /
0 A/m (which is equal) is returned.

# Serialization and deserialization

A [`FerromagneticPermeability`] has no hidden fields and is therefore serialized
as a struct of two [`AkimaSpline`]s. It can be deserialized from the serialized
representation of the following structs:

1) Its own "native" representation
2) A [`MagnetizationCurve`]
2) A [`PolarizationCurve`]

In case of the latter two, the corresponding structs are deserialized first
directly and then the constructors
[`from_magnetization`](FerromagneticPermeability::from_magnetization) or
[`from_polarization`](FerromagneticPermeability::from_polarization) are used to
create a [`FerromagneticPermeability`] instance.
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "serde_impl::FerromagneticPermeabilityDeEnum")
)]
pub struct FerromagneticPermeability {
    /// Spline representing the function `f(H) = µr`.
    pub from_field_strength: AkimaSpline,
    /// Spline representing the function `f(B) = µr`.
    pub from_flux_density: AkimaSpline,
}

impl FerromagneticPermeability {
    /**
    Constructs a [`FerromagneticPermeability`] from a [`MagnetizationCurve`].

    This process can fail for the reasons described in the [`InvalidInputData`]
    error enum.
     */
    pub fn from_magnetization(raw_curve: MagnetizationCurve) -> Result<Self, InvalidInputData> {
        let (field_strength, flux_density) = sample_bh_curve(
            raw_curve.field_strength.as_slice(),
            raw_curve.flux_density.as_slice(),
            0.02,
        )?;

        // ==========================================================================
        // Start of curve creation

        // Calculate relative permeability
        let mut induction: Vec<f64> = Vec::with_capacity(field_strength.len());
        let mut permeability: Vec<f64> = Vec::with_capacity(field_strength.len());
        let mut field_strength_spline: Vec<f64> = Vec::with_capacity(field_strength.len());

        for (hi, bi) in field_strength
            .iter()
            .map(|value| value.get::<ampere_per_meter>().clone())
            .zip(
                flux_density
                    .iter()
                    .map(|value| value.get::<tesla>().clone()),
            )
        {
            if hi != 0.0 {
                // Adjust for the iron fill factor
                let b_red = bi * raw_curve.iron_fill_factor
                    + (1.0 - raw_curve.iron_fill_factor) * hi * VACUUM_PERMEABILITY_UNITLESS;

                // Calculate with the reduced flux density
                let mu_r = b_red / (hi * VACUUM_PERMEABILITY_UNITLESS);
                field_strength_spline.push(hi);
                induction.push(b_red);
                permeability.push(mu_r);
            }
        }

        let mut idx_max = None;
        let mut min_value = std::f64::NEG_INFINITY;
        for (idx, value) in permeability.iter().enumerate() {
            if *value > min_value {
                min_value = *value;
                idx_max = Some(idx);
            }
        }
        let idx_max = idx_max.expect("Guaranteed to have at least one value by the constructor");

        // Remove all values "left" of idx_max
        let field_strength_right_of_maximum = &field_strength_spline[idx_max..];
        let induction_right_of_maximum = &induction[idx_max..];
        let permeability_right_of_maximum = &permeability[idx_max..];
        let field_strength = field_strength_right_of_maximum.to_vec();
        let induction = induction_right_of_maximum.to_vec();
        let mut permeability = permeability_right_of_maximum.to_vec();

        // Modify mu_r(B) to ensure strictly decreasing behaviour.
        if permeability.len() > 2 {
            for idx in (0..(permeability.len() - 2)).rev() {
                if permeability[idx] < permeability[idx + 1] {
                    let m = (permeability[idx + 1] - permeability[idx + 2])
                        / (induction[idx + 1] - induction[idx + 2]);

                    // Calculate the new y-value with the gradient
                    permeability[idx] =
                        permeability[idx + 1] + m * (induction[idx + 1] - induction[idx + 2]);
                }
            }
        }

        // Extrapolation function for induction values larger than induction[end].
        let induction_1 = *induction
            .last()
            .expect("Guaranteed to have at least one value by the constructor");
        let induction_2 = 100.0;
        let permeability_1 = *permeability
            .last()
            .expect("Guaranteed to have at least one value by the constructor");
        let permeability_2 = 1.0;
        let field_strength_1 = induction_1 / (VACUUM_PERMEABILITY_UNITLESS * permeability_1);
        let field_strength_2 = induction_2 / (VACUUM_PERMEABILITY_UNITLESS * permeability_2);

        // Create the mu_r(field_strength)-curce
        let mr = (permeability_2 - permeability_1) / (field_strength_2 - field_strength_1);

        // Extrapolate with a horizontal line from the permeability maximum to the left
        let ml = 0.0;

        let extrapl = Some(vec![ml]);
        let extrapr = Some(vec![mr]);
        let from_field_strength =
            AkimaSpline::new(field_strength, permeability.clone(), extrapl, extrapr)
                .expect("values are guaranteed to be in ascending order");

        // Create the mu_r(flux_density)-curce
        let mr = (permeability_2 - permeability_1) / (induction_2 - induction_1);

        // Extrapolate with a horizontal line from the permeability maximum to the left
        let ml = 0.0;

        let extrapl = Some(vec![ml]);
        let extrapr = Some(vec![mr]);
        let from_flux_density = AkimaSpline::new(induction, permeability, extrapl, extrapr)?;

        return Ok(Self {
            from_field_strength,
            from_flux_density,
        });
    }

    /**
    Constructs a [`FerromagneticPermeability`] from a [`PolarizationCurve`].

    This process can fail for the reasons described in the [`InvalidInputData`]
    error enum.
     */
    pub fn from_polarization(raw_curve: PolarizationCurve) -> Result<Self, InvalidInputData> {
        return raw_curve.try_into();
    }

    /**
    Returns the relative permeability for the given magnetic field strength or
    flux density.
     */
    pub fn get<T: FieldStrengthOrFluxDensity>(&self, value: T) -> f64 {
        return value.permeability(&self);
    }
}

#[cfg_attr(feature = "serde", typetag::serde)]
impl IsQuantityFunction for FerromagneticPermeability {
    fn call(&self, influencing_factors: &[DynQuantity<f64>]) -> DynQuantity<f64> {
        for f in influencing_factors {
            if f.unit == Unit::from(PredefUnit::MagneticFieldStrength) {
                return self
                    .from_field_strength
                    .eval_infallible(f.value.abs())
                    .clamp(1.0, INFINITY)
                    .into();
            } else if f.unit == Unit::from(PredefUnit::MagneticFluxDensity) {
                return self
                    .from_flux_density
                    .eval_infallible(f.value.abs())
                    .clamp(1.0, INFINITY)
                    .into();
            }
        }
        return self.from_flux_density.eval_infallible(0.0).into();
    }
}

/**
Sealed helper trait for [`FerromagneticPermeability::get`].

This sealed trait is implemented for [`MagneticFieldStrength`] and
[`MagneticFluxDensity`] to enable [`FerromagneticPermeability::get`] to receive
either of the two quantities as arguments. It is not meant to be implemented for
any other types or to be used on its own.
 */
pub trait FieldStrengthOrFluxDensity: private::Sealed {
    /**
    Returns the relative `permeability` for `self`.

    This function is used to implement [`FerromagneticPermeability::get`] and
    not meant to be used on its own.
     */
    fn permeability(self, permeability: &FerromagneticPermeability) -> f64;
}

impl private::Sealed for MagneticFieldStrength {}

impl FieldStrengthOrFluxDensity for MagneticFieldStrength {
    fn permeability(self, permeability: &FerromagneticPermeability) -> f64 {
        let raw = self.get::<ampere_per_meter>();
        return permeability.from_field_strength.eval_infallible(raw);
    }
}

impl private::Sealed for MagneticFluxDensity {}

impl FieldStrengthOrFluxDensity for MagneticFluxDensity {
    fn permeability(self, permeability: &FerromagneticPermeability) -> f64 {
        let raw = self.get::<tesla>();
        return permeability.from_flux_density.eval_infallible(raw);
    }
}

mod private {
    pub trait Sealed {}
}

/**
A collection of datapoints representing the magnetization curve of a material.

This curve contains `B` / `H` datapoints, whose quotient according to the
equation `B = µ0 * µr * H` is the (absolute) permeability `µ0 * µr` for this
flux density / field strength. From these datapoints, a
[`FerromagneticPermeability`] struct can be obtained using the [`TryFrom`]
implementation or the [`FerromagneticPermeability::from_magnetization`] method.

Data curves for ferromagnetic material is usually obtained measuring massive
material blocks. However, the magnetic cores of electrical machines are often
"stacked" from small material sheets which have an insulation layer between
them to reduce eddy currents. The insulation layer has a relative permeability
of roughly 1, which is why the calculated `µr` has to be adjusted depending on
the ratio between the insulation layer and the ferromagnetic material. This
ratio is called the "iron fill factor", which can be between 1 (massive
material, no layer) and 0 (only layer). This iron fill factor has to be
specified as an argument to [`MagnetizationCurve::new`]. Usually, its value is
between 0.98 and 0.95, depending on the thickness of the sheet itself.
 */
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MagnetizationCurve {
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_vec_of_quantities")
    )]
    field_strength: Vec<MagneticFieldStrength>,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_vec_of_quantities")
    )]
    flux_density: Vec<MagneticFluxDensity>,
    iron_fill_factor: f64,
}

impl MagnetizationCurve {
    /**
    Returns a new [`PolarizationCurve`], provided that the given input data is
    valid. This is the case of none of the error cases of the
    [`InvalidInputData`] are fulfilled.

    # Examples

    ```
    use stem_material::*;
    use uom::si::magnetic_field_strength::ampere_per_meter;
    use uom::si::magnetic_flux_density::tesla;

    // Valid input data
    assert!(MagnetizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0), MagneticFieldStrength::new::<ampere_per_meter>(150.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        0.95,
    ).is_ok());

    // Unequal vector length
    assert!(MagnetizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        0.95,
    ).is_err());


    // Invalid iron fill factor
    assert!(MagnetizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0), MagneticFieldStrength::new::<ampere_per_meter>(150.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        1.1,
    ).is_err());
    ```
     */
    pub fn new(
        field_strength: Vec<MagneticFieldStrength>,
        flux_density: Vec<MagneticFluxDensity>,
        iron_fill_factor: f64,
    ) -> Result<Self, InvalidInputData> {
        let data = MagnetizationCurve {
            field_strength,
            flux_density,
            iron_fill_factor,
        };
        data.check()?;
        return Ok(data);
    }

    // Check the integrity of the data
    fn check(&self) -> Result<(), InvalidInputData> {
        if self.iron_fill_factor > 1.0 || self.iron_fill_factor < 0.0 {
            return Err(InvalidInputData::IronFillFactor(self.iron_fill_factor));
        }
        if self.field_strength.len() != self.flux_density.len() {
            return Err(InvalidInputData::IneqNumElementsFluxDensity {
                field_strength: self.field_strength.len(),
                flux_density: self.flux_density.len(),
            });
        }
        return Ok(());
    }
}

impl TryFrom<MagnetizationCurve> for FerromagneticPermeability {
    type Error = InvalidInputData;

    fn try_from(value: MagnetizationCurve) -> Result<Self, Self::Error> {
        return FerromagneticPermeability::from_magnetization(value);
    }
}

/**
A collection of datapoints representing the polarization curve of a material.

The polarization `J` is related to the flux density `B`, the field strength `H`
and the [vacuum permability](VACUUM_PERMEABILITY) `µ0` via the following
equation:

`J = B - µ0 * H`

As such, this struct is essentially an alternative representation of a
[`MagnetizationCurve`] and can be easily converted into it using the [`TryFrom`]
implementation. As with the [`MagnetizationCurve`], the main purpose of this
struct is to serve as a building block for a [`FerromagneticPermeability`]
struct.

Data curves for ferromagnetic material is usually obtained measuring massive
material blocks. However, the magnetic cores of electrical machines are often
"stacked" from small material sheets which have an insulation layer between
them to reduce eddy currents. The insulation layer has a relative permeability
of roughly 1, which is why the calculated `µr` has to be adjusted depending on
the ratio between the insulation layer and the ferromagnetic material. This
ratio is called the "iron fill factor", which can be between 1 (massive
material, no layer) and 0 (only layer). This iron fill factor has to be
specified as an argument to [`PolarizationCurve::new`]. Usually, its value is
between 0.98 and 0.95, depending on the thickness of the sheet itself.
 */
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolarizationCurve {
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_vec_of_quantities")
    )]
    field_strength: Vec<MagneticFieldStrength>,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_vec_of_quantities")
    )]
    polarization: Vec<MagneticFluxDensity>,
    iron_fill_factor: f64,
}

impl PolarizationCurve {
    /**
    Returns a new [`PolarizationCurve`], provided that the given input data is
    valid. This is the case of none of the error cases of the
    [`InvalidInputData`] are fulfilled.

    # Examples

    ```
    use stem_material::*;
    use uom::si::magnetic_field_strength::ampere_per_meter;
    use uom::si::magnetic_flux_density::tesla;

    // Valid input data
    assert!(PolarizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0), MagneticFieldStrength::new::<ampere_per_meter>(150.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        0.95,
    ).is_ok());

    // Unequal vector length
    assert!(PolarizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        0.95,
    ).is_err());


    // Invalid iron fill factor
    assert!(PolarizationCurve::new(
        vec![MagneticFieldStrength::new::<ampere_per_meter>(100.0),MagneticFieldStrength::new::<ampere_per_meter>(150.0)],
        vec![MagneticFluxDensity::new::<tesla>(0.5), MagneticFluxDensity::new::<tesla>(0.6)],
        1.1,
    ).is_err());
    ```
     */
    pub fn new(
        field_strength: Vec<MagneticFieldStrength>,
        polarization: Vec<MagneticFluxDensity>,
        iron_fill_factor: f64,
    ) -> Result<Self, InvalidInputData> {
        let data = PolarizationCurve {
            field_strength,
            polarization,
            iron_fill_factor,
        };
        data.check()?;
        return Ok(data);
    }

    // Check the integrity of the data
    fn check(&self) -> Result<(), InvalidInputData> {
        if self.iron_fill_factor > 1.0 || self.iron_fill_factor < 0.0 {
            return Err(InvalidInputData::IronFillFactor(self.iron_fill_factor));
        }
        if self.field_strength.len() != self.polarization.len() {
            return Err(InvalidInputData::IneqNumElementsPolarization {
                field_strength: self.field_strength.len(),
                polarization: self.polarization.len(),
            });
        }
        return Ok(());
    }
}

impl TryFrom<PolarizationCurve> for MagnetizationCurve {
    type Error = InvalidInputData;

    fn try_from(value: PolarizationCurve) -> Result<Self, InvalidInputData> {
        // Calculate the flux density from the polarization
        let mut flux_density = value.polarization;
        flux_density
            .iter_mut()
            .zip(value.field_strength.iter())
            .for_each(|(b, h)| {
                *b = *b + *h * *VACUUM_PERMEABILITY;
            });

        let data = MagnetizationCurve {
            field_strength: value.field_strength,
            flux_density,
            iron_fill_factor: value.iron_fill_factor,
        };
        data.check()?;
        return Ok(data);
    }
}

impl TryFrom<PolarizationCurve> for FerromagneticPermeability {
    type Error = InvalidInputData;

    fn try_from(value: PolarizationCurve) -> Result<Self, InvalidInputData> {
        let magnetization_curve = MagnetizationCurve::try_from(value)?;
        return magnetization_curve.try_into();
    }
}

/**
Errors which can occur when attempting to convert a [`MagnetizationCurve`] or
[`PolarizationCurve`] into a [`FerromagneticPermeability`].
 */
#[derive(Debug)]
pub enum InvalidInputData {
    /// The specified iron fill factor is not between 0 and 1 (0 % and 100 %).
    IronFillFactor(f64),
    /**
    The given vectors for magnetic field strength and flux density did not have
    the same length. This error can only be returned when starting from a
    [`MagnetizationCurve`].
     */
    IneqNumElementsFluxDensity {
        /// Length of the field strength vector
        field_strength: usize,
        /// Length of the flux density vector
        flux_density: usize,
    },
    /**
    The given vectors for magnetic field strength and polarization did not have
    the same length. This error can only be returned when starting from a
    [`PolarizationCurve`].
     */
    IneqNumElementsPolarization {
        /// Length of the field strength vector
        field_strength: usize,
        /// Length of the polarization vector
        polarization: usize,
    },
    /// Building one of the [`AkimaSpline`]s failed.
    AkimaBuildError(akima_spline::BuildError),
}

impl From<akima_spline::BuildError> for InvalidInputData {
    fn from(value: akima_spline::BuildError) -> Self {
        return Self::AkimaBuildError(value);
    }
}

impl std::fmt::Display for InvalidInputData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidInputData::IronFillFactor(value) => write!(
                f,
                "iron fill factor must be between 0 and 1 (0 % and 100 %), is {value}."
            ),
            InvalidInputData::IneqNumElementsFluxDensity {
                field_strength,
                flux_density,
            } => write!(
                f,
                "got {field_strength} values for field strength, but
                {flux_density} values for flux density (should be equal)."
            ),
            InvalidInputData::IneqNumElementsPolarization {
                field_strength,
                polarization,
            } => write!(
                f,
                "got {field_strength} values for field strength, but
                {polarization} values for polarization (should be equal)."
            ),
            InvalidInputData::AkimaBuildError(error) => return error.fmt(f),
        }
    }
}

impl std::error::Error for InvalidInputData {}

/**
Sample the given BH curve so that the maximum permeability change between two
support points is equal / less than the given tolerance.
 */
fn sample_bh_curve(
    field_strength: &[MagneticFieldStrength],
    flux_density: &[MagneticFluxDensity],
    change_tol: f64,
) -> Result<(Vec<MagneticFieldStrength>, Vec<MagneticFluxDensity>), InvalidInputData> {
    // Intial sample step width of 10 A/m
    let sample_step_width = MagneticFieldStrength::new::<ampere_per_meter>(10.0);

    let max_field_strength = field_strength
        .iter()
        .cloned()
        .reduce(|first, second| if first > second { first } else { second })
        .expect("must have at least one element");

    // Create a B(H) curve
    let extrapl = Some(vec![VACUUM_PERMEABILITY_UNITLESS]);
    let extrapr = Some(vec![VACUUM_PERMEABILITY_UNITLESS]);
    let bh_curve = AkimaSpline::new(
        field_strength
            .iter()
            .map(|val| val.get::<ampere_per_meter>())
            .collect(),
        flux_density.iter().map(|val| val.get::<tesla>()).collect(),
        extrapl,
        extrapr,
    )?;

    let mut h_sampled: Vec<MagneticFieldStrength> = Vec::with_capacity(1000);
    let mut b_sampled: Vec<MagneticFluxDensity> = Vec::with_capacity(1000);

    // Create the initial values
    h_sampled.push(MagneticFieldStrength::new::<ampere_per_meter>(0.0));
    b_sampled.push(MagneticFluxDensity::new::<tesla>(0.0));
    h_sampled.push(sample_step_width);
    b_sampled.push(MagneticFluxDensity::new::<tesla>(
        bh_curve.eval_infallible(sample_step_width.get::<ampere_per_meter>()),
    ));

    let mut current_field_strength = 2.0 * sample_step_width;

    while current_field_strength < max_field_strength {
        let mu_prev = b_sampled
            .last()
            .expect("b_sampled has at least one element")
            .clone()
            / h_sampled
                .last()
                .expect("h_sampled has at least one element")
                .clone();
        let current_flux_density = MagneticFluxDensity::new::<tesla>(
            bh_curve.eval_infallible(current_field_strength.get::<ampere_per_meter>()),
        );
        let mu_curr = current_flux_density / current_field_strength;

        // If the tolerance was exceeded, keep the current values as support points.
        // Otherwise, skip the current values
        if f64::from((mu_prev - mu_curr).abs() / mu_prev) > change_tol {
            h_sampled.push(current_field_strength);
            b_sampled.push(current_flux_density);
        }
        current_field_strength = current_field_strength + sample_step_width;
    }

    return Ok((h_sampled, b_sampled));
}

#[cfg(feature = "serde")]
mod serde_impl {
    use deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError;

    use super::*;

    #[derive(Deserialize)]
    pub(super) struct FerromagneticPermeabilityDeserializeAlias {
        from_field_strength: AkimaSpline,
        from_flux_density: AkimaSpline,
    }

    #[derive(DeserializeUntaggedVerboseError)]
    pub(super) enum FerromagneticPermeabilityDeEnum {
        FerromagneticPermeability(FerromagneticPermeabilityDeserializeAlias),
        MagnetizationCurve(MagnetizationCurve),
        PolarizationCurve(PolarizationCurve),
    }

    impl TryFrom<FerromagneticPermeabilityDeEnum> for FerromagneticPermeability {
        type Error = InvalidInputData;

        fn try_from(value: FerromagneticPermeabilityDeEnum) -> Result<Self, InvalidInputData> {
            match value {
                FerromagneticPermeabilityDeEnum::FerromagneticPermeability(val) => {
                    Ok(FerromagneticPermeability {
                        from_field_strength: val.from_field_strength,
                        from_flux_density: val.from_flux_density,
                    })
                }
                FerromagneticPermeabilityDeEnum::MagnetizationCurve(val) => {
                    FerromagneticPermeability::try_from(val)
                }
                FerromagneticPermeabilityDeEnum::PolarizationCurve(val) => {
                    FerromagneticPermeability::try_from(val)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use approx;

    #[test]
    fn test_sample_bh_curve() {
        let field_strength: Vec<MagneticFieldStrength> = vec![
            0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83,
            179.45, 276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16,
            45905.16, 69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
        ]
        .into_iter()
        .map(MagneticFieldStrength::new::<ampere_per_meter>)
        .collect();
        let flux_density: Vec<MagneticFluxDensity> = vec![
            0.0, 0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701,
            1.0672, 1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674,
            2.1674, 2.2674, 2.3674, 2.4674, 2.4720,
        ]
        .into_iter()
        .map(MagneticFluxDensity::new::<tesla>)
        .collect();

        let (h, b) =
            sample_bh_curve(field_strength.as_slice(), flux_density.as_slice(), 0.02).unwrap();

        let len = 300;
        assert_eq!(h.len(), len);
        assert_eq!(h.len(), len);

        // Field strength
        approx::assert_abs_diff_eq!(h[0].get::<ampere_per_meter>(), 0.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(h[1].get::<ampere_per_meter>(), 10.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(h[2].get::<ampere_per_meter>(), 20.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(h[50].get::<ampere_per_meter>(), 580.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(h[150].get::<ampere_per_meter>(), 7040.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(h[299].get::<ampere_per_meter>(), 217110.0, epsilon = 0.001);

        // Flux density
        approx::assert_abs_diff_eq!(b[0].get::<tesla>(), 0.0, epsilon = 0.001);
        approx::assert_abs_diff_eq!(b[1].get::<tesla>(), 0.08142, epsilon = 0.001);
        approx::assert_abs_diff_eq!(b[2].get::<tesla>(), 0.17399, epsilon = 0.001);
        approx::assert_abs_diff_eq!(b[50].get::<tesla>(), 1.35845, epsilon = 0.001);
        approx::assert_abs_diff_eq!(b[150].get::<tesla>(), 1.66712, epsilon = 0.001);
        approx::assert_abs_diff_eq!(b[299].get::<tesla>(), 2.46926, epsilon = 0.001);
    }
}
