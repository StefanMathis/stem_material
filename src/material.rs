#![doc = include_str!("../README.md")]
// #![deny(missing_docs)]

#[cfg(feature = "serde")]
use serde_mosaic::{
    DatabaseEntry,
    serde::{Deserialize, Serialize},
};
use uom::si::specific_power::watt_per_kilogram;
use var_quantity::VarQuantity;

#[cfg(feature = "serde")]
use std::ffi::OsStr;

use std::{fmt::Debug, mem};
pub use uom;
pub use uom::si::{
    electrical_resistivity::ohm_meter, f64::*, frequency::hertz, heat_capacity::joule_per_kelvin,
    magnetic_field_strength::ampere_per_meter, magnetic_flux_density::tesla,
    magnetic_permeability::henry_per_meter, mass_density::kilogram_per_cubic_meter,
    specific_heat_capacity::joule_per_kilogram_kelvin, thermal_conductivity::watt_per_meter_kelvin,
    thermodynamic_temperature::degree_celsius,
};

use crate::iron_losses::*;
use crate::relative_permeability::*;

/**
SI-value of the vacuum magnetic permeability (4π*1e-7 N*A²) without units.

This value is based on the former ampere definition used until 2019. Since
the new definition of the ampere, the vacuum magnetic permeability is actually
4π * 0.99999999987(16) * 1e-7 H/m. This deviation is within the measurement
uncertainty and cannot be reasonably be represented by [`f64`] anyway
Therefore, the stem framework still uses the old definition.
See [https://en.wikipedia.org/wiki/Vacuum_permeability].
 */
pub const VACUUM_PERMEABILITY_UNITLESS: f64 = 4.0 * std::f64::consts::PI * 1e-7;

lazy_static::lazy_static! {
    /**
    SI-value of the vacuum magnetic permeability (4π*1e-7 N*A²) with units.
    See [`VACUUM_PERMEABILITY_UNITLESS`] for more.
     */
    pub static ref VACUUM_PERMEABILITY: MagneticPermeability =
        MagneticPermeability::new::<henry_per_meter>(
            VACUUM_PERMEABILITY_UNITLESS
        );
}

/**
A substance which constitutes an object, e.g. a magnet or a wire in the context
of [stem](github.com/StefanMathis/stem_book).

This struct is literally just the sum of its parts: It represents a material as
a collection of properties such as its mass density, electrical resistivity or
heat capacity. Each of its fields can be accessed directly or via its getter
and setter methods. Since all fields are defined using the [uom] crate, the
type system ensures that the output value is always given in SI units. Every
property is thought to be homogeneous (e.g. does not change depending on the
orientation of the material) unless explicitly stated otherwise in the field
description. All property fields use the [`VarQuantity`] enum which can
represent the change of properties due to external factors (e.g. the increase of
electrical resistivity with temperature).

It is important to note that the material should always return reasonable values
for physical properties, otherwise calculations might return non-physical
results or even fail completely (for example, returning a negative resistivity
would result in negative losses, which would mean that a motor gets colder when
its current gets larger). As a general guideline, the property getter function
should never return a negative value for any conditions. Since the quantitity
functions used in [`VarQuantity`] can be arbitrary Rust functions, the stem
framework cannot guard against this.

A [`Material`] is used as part of motor components within
[stem](github.com/StefanMathis/stem_book). For example, a simple definition of
a wire would look like this:

```rust
pub use uom::si::f64::*;
use stem_material::Material;

trait Wire {
    fn resistance(&self, temperature: ThermodynamicTemperature) -> ElectricalResistance;
}

struct RoundWire {
    cross_surface: Area,
    length: Length,
    material: Material,
}

impl Wire for RoundWire {
    fn resistance(&self, temperature: ThermodynamicTemperature) -> ElectricalResistance {
        let resistivity = self.material.electrical_resistivity.get(&[temperature.into()]);
        return self.length * resistivity / self.cross_surface;
    }
}
```
The `Wire` trait would then be used inside stem to calculate the resistance,
using the wire temperature as a `condition` input to [`VarQuantity::get`].

[`Material`] provides a [`Default`] implementation with reasonable defaults for
each physical property to allow incremental construction. These default values
are also used when a property field is missing during deserialization. It also
implements [`DatabaseEntry`] which is very useful when maintaining e.g. a
database of motors: Commonly used materials such as copper for the wire only
need to be defined once and can then be reused across all motors.
*/
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(deny_unknown_fields))]
pub struct Material {
    /// Name of `self`, e.g. "Copper"
    pub name: String,

    /// Relative permeability of `self`. For vacuum, this value is 1.
    ///
    /// Defaults to 1.
    #[cfg_attr(feature = "serde", serde(default = "default_relative_permeability"))]
    pub relative_permeability: RelativePermeability,

    /// Specific iron losses of `self`.
    ///
    /// Defaults to 0 W/kg.
    #[cfg_attr(feature = "serde", serde(default = "default_iron_losses"))]
    pub iron_losses: IronLosses,

    /// Remanence of `self`. This value is usually zero except for permanent
    /// magnets. For permanent magnets, this value is expected to be the remance
    /// in the magnetization / easy axis.
    ///
    /// Defaults to 0 T.
    #[cfg_attr(feature = "serde", serde(default = "default_remanence"))]
    pub remanence: VarQuantity<MagneticFluxDensity>,

    /// Intrinsic coercivity of `self`. This value is usually zero except for
    /// permanent magnets. For permanent magnets, this value is expected to be
    /// the coercitivity in the magnetization / easy axis.
    ///
    /// Defaults to 0 A/m.
    #[cfg_attr(feature = "serde", serde(default = "default_intrinsic_coercivity"))]
    pub intrinsic_coercivity: VarQuantity<MagneticFieldStrength>,

    /// Electrical resistivity of `self`. For isolators, this value is infinity,
    /// for superconductors, it is zero.
    ///
    /// Defaults to infinity ohm*meter.
    #[cfg_attr(feature = "serde", serde(default = "default_electrical_resistivity"))]
    pub electrical_resistivity: VarQuantity<ElectricalResistivity>,

    /// Mass density of `self`.
    ///
    /// Defaults to 1000 kg/m³.
    #[cfg_attr(feature = "serde", serde(default = "default_mass_density"))]
    pub mass_density: VarQuantity<MassDensity>,

    /// Specific heat capacity of `self`.
    ///
    /// Defaults to 0 J/(kg * K).
    #[cfg_attr(feature = "serde", serde(default = "default_heat_capacity"))]
    pub heat_capacity: VarQuantity<SpecificHeatCapacity>,

    /// Thermal conductivity of `self`.
    ///
    /// Defaults to 0 W/(m * K).
    #[cfg_attr(feature = "serde", serde(default = "default_thermal_conductivity"))]
    pub thermal_conductivity: VarQuantity<ThermalConductivity>,
}

impl Material {
    /// Returns the name of `self`.
    pub fn name(&self) -> &str {
        return &self.name;
    }

    /// Sets a new name for `self` and returns the old one.
    pub fn set_name(&mut self, name: String) -> String {
        return mem::replace(&mut self.name, name);
    }

    /// Returns the relative permeability of `self`.
    pub fn relative_permeability(&self) -> &RelativePermeability {
        return &self.relative_permeability;
    }

    /// Sets a new relative permeability and returns the old one.
    pub fn set_relative_permeability(
        &mut self,
        property: RelativePermeability,
    ) -> RelativePermeability {
        return mem::replace(&mut self.relative_permeability, property);
    }

    /// Returns the specific iron losses of `self`.
    pub fn iron_losses(&self) -> &IronLosses {
        return &self.iron_losses;
    }

    /// Sets new specific iron losses and returns the old ones.
    pub fn set_iron_losses(&mut self, property: IronLosses) -> IronLosses {
        return mem::replace(&mut self.iron_losses, property);
    }

    /// Returns the remanence of `self`.
    pub fn remanence(&self) -> &VarQuantity<MagneticFluxDensity> {
        return &self.remanence;
    }

    /// Sets a new remanence and returns the old one.
    pub fn set_remanence(
        &mut self,
        property: VarQuantity<MagneticFluxDensity>,
    ) -> VarQuantity<MagneticFluxDensity> {
        return mem::replace(&mut self.remanence, property);
    }

    /// Returns the intrinsic coercivity of `self`.
    pub fn intrinsic_coercivity(&self) -> &VarQuantity<MagneticFieldStrength> {
        return &self.intrinsic_coercivity;
    }

    /// Sets a new intrinsic coercivity and returns the old one.
    pub fn set_intrinsic_coercivity(
        &mut self,
        property: VarQuantity<MagneticFieldStrength>,
    ) -> VarQuantity<MagneticFieldStrength> {
        return mem::replace(&mut self.intrinsic_coercivity, property);
    }

    /// Returns the electrical resistivity of `self`.
    pub fn electrical_resistivity(&self) -> &VarQuantity<ElectricalResistivity> {
        return &self.electrical_resistivity;
    }

    /// Sets a new electrical resistivity and returns the old one.
    pub fn set_electrical_resistivity(
        &mut self,
        property: VarQuantity<ElectricalResistivity>,
    ) -> VarQuantity<ElectricalResistivity> {
        return mem::replace(&mut self.electrical_resistivity, property);
    }

    /// Returns the mass density of `self`.
    pub fn mass_density(&self) -> &VarQuantity<MassDensity> {
        return &self.mass_density;
    }

    /// Sets a new mass density and returns the old one.
    pub fn set_mass_density(
        &mut self,
        property: VarQuantity<MassDensity>,
    ) -> VarQuantity<MassDensity> {
        return mem::replace(&mut self.mass_density, property);
    }

    /// Returns the specific heat capacity of `self`.
    pub fn heat_capacity(&self) -> &VarQuantity<SpecificHeatCapacity> {
        return &self.heat_capacity;
    }

    /// Sets a new specific heat capacity and returns the old one.
    pub fn set_heat_capacity(
        &mut self,
        property: VarQuantity<SpecificHeatCapacity>,
    ) -> VarQuantity<SpecificHeatCapacity> {
        return mem::replace(&mut self.heat_capacity, property);
    }

    /// Returns the thermal conductivity of `self`.
    pub fn thermal_conductivity(&self) -> &VarQuantity<ThermalConductivity> {
        return &self.thermal_conductivity;
    }

    /// Sets a new thermal conductivity and returns the old one.
    pub fn set_thermal_conductivity(
        &mut self,
        property: VarQuantity<ThermalConductivity>,
    ) -> VarQuantity<ThermalConductivity> {
        return mem::replace(&mut self.thermal_conductivity, property);
    }
}

impl Default for Material {
    fn default() -> Self {
        return Material {
            name: "default_name".to_string(),
            relative_permeability: default_relative_permeability(),
            iron_losses: default_iron_losses(),
            remanence: default_remanence(),
            intrinsic_coercivity: default_intrinsic_coercivity(),
            electrical_resistivity: default_electrical_resistivity(),
            mass_density: default_mass_density(),
            heat_capacity: default_heat_capacity(),
            thermal_conductivity: default_thermal_conductivity(),
        };
    }
}

#[cfg(feature = "serde")]
#[typetag::serde]
impl DatabaseEntry for Material {
    fn name(&self) -> &OsStr {
        self.name.as_ref()
    }
}

fn default_relative_permeability() -> RelativePermeability {
    return RelativePermeability::Constant(1.0);
}

fn default_iron_losses() -> IronLosses {
    return IronLosses::Constant(SpecificPower::new::<watt_per_kilogram>(0.0));
}

fn default_remanence() -> VarQuantity<MagneticFluxDensity> {
    return VarQuantity::Constant(MagneticFluxDensity::new::<tesla>(0.0));
}

fn default_intrinsic_coercivity() -> VarQuantity<MagneticFieldStrength> {
    return VarQuantity::Constant(MagneticFieldStrength::new::<ampere_per_meter>(0.0));
}

fn default_electrical_resistivity() -> VarQuantity<ElectricalResistivity> {
    return VarQuantity::Constant(ElectricalResistivity::new::<ohm_meter>(std::f64::INFINITY));
}

fn default_mass_density() -> VarQuantity<MassDensity> {
    return VarQuantity::Constant(MassDensity::new::<kilogram_per_cubic_meter>(1000.0));
}

fn default_heat_capacity() -> VarQuantity<SpecificHeatCapacity> {
    return VarQuantity::Constant(SpecificHeatCapacity::new::<joule_per_kilogram_kelvin>(0.0));
}

fn default_thermal_conductivity() -> VarQuantity<ThermalConductivity> {
    return VarQuantity::Constant(ThermalConductivity::new::<watt_per_meter_kelvin>(0.0));
}
