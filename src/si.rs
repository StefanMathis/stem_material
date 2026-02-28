/*!
The [stem_material crate](crate) uses the strongly-typed SI quantities provided
by the [uom](crate::uom) crate. This module reexports the commonly used types
and units for electromagnetics.

If a unit is not directly reexported, the path `stem_material::uom_prelude`
also acts as an alias of `stem_material::uom::si`.

# Example

```
use stem_material::uom::si::f64::*;
use stem_material::uom::si::length::millimeter;
use stem_material::uom::si::area::square_millimeter;
use stem_material::uom::si::electrical_resistance::ohm;
use stem_material::uom::si::thermodynamic_temperature::degree_celsius;
use stem_material::uom::si::temperature_interval::kelvin;
```
becomes
```
use stem_material::prelude::*;

// Since the kelvin type appears in both `thermodynamic_temperature` and
// `temperature_interval`, the latter one needs to be reexported explictly,
// shadowing `uom::si::thermodynamic_temperature::kelvin`.
use stem_material::prelude::si::temperature_interval::kelvin;
```
 */

// Quantities
pub use var_quantity::uom::si;
pub use var_quantity::uom::si::f64::*;
pub use var_quantity::uom::si::ratio::ratio;

// Length and mass
pub use var_quantity::uom::si::area::{
    square_centimeter, square_kilometer, square_meter, square_micrometer, square_millimeter,
};
pub use var_quantity::uom::si::length::{centimeter, kilometer, meter, micrometer, millimeter};
pub use var_quantity::uom::si::mass::{gram, kilogram, microgram, milligram, ton};
pub use var_quantity::uom::si::mass_density::{
    gram_per_cubic_meter, kilogram_per_cubic_meter, microgram_per_cubic_meter,
    milligram_per_cubic_meter, ton_per_cubic_meter,
};
pub use var_quantity::uom::si::moment_of_inertia::kilogram_square_meter;
pub use var_quantity::uom::si::specific_volume::{
    cubic_centimeter_per_gram, cubic_meter_per_gram, cubic_meter_per_kilogram,
};
pub use var_quantity::uom::si::volume::{
    cubic_centimeter, cubic_kilometer, cubic_meter, cubic_micrometer, cubic_millimeter,
};

// Time
pub use var_quantity::uom::si::frequency::{
    gigahertz, hertz, kilohertz, megahertz, microhertz, millihertz,
};
pub use var_quantity::uom::si::time::{day, hour, microsecond, millisecond, minute, second, year};

// Velocity and acceleration
pub use var_quantity::uom::si::acceleration::{
    centimeter_per_second_squared, kilometer_per_second_squared, meter_per_second_squared,
    millimeter_per_minute_squared, millimeter_per_second_squared, standard_gravity,
};
pub use var_quantity::uom::si::angular_acceleration::radian_per_second_squared;
pub use var_quantity::uom::si::angular_velocity::{
    radian_per_second, revolution_per_hour, revolution_per_minute, revolution_per_second,
};
pub use var_quantity::uom::si::velocity::{
    centimeter_per_second, kilometer_per_hour, kilometer_per_second, meter_per_second,
    micrometer_per_second, millimeter_per_minute, millimeter_per_second,
};

// Force and torque
pub use var_quantity::uom::si::force::{
    giganewton, kilonewton, meganewton, micronewton, millinewton, newton,
};
pub use var_quantity::uom::si::torque::{
    newton_centimeter, newton_kilometer, newton_meter, newton_micrometer, newton_millimeter,
};

// Power and energy
pub use var_quantity::uom::si::energy::{
    gigajoule, joule, kilojoule, megajoule, microjoule, millijoule,
};
pub use var_quantity::uom::si::power::{gigawatt, kilowatt, megawatt, microwatt, milliwatt, watt};
pub use var_quantity::uom::si::specific_power::{
    gigawatt_per_kilogram, kilowatt_per_kilogram, megawatt_per_kilogram, microwatt_per_kilogram,
    milliwatt_per_kilogram, watt_per_kilogram,
};
// Magnetism
pub use var_quantity::uom::si::reciprocal_length::{
    reciprocal_centimeter, reciprocal_kilometer, reciprocal_micrometer, reciprocal_millimeter,
};

// Current and voltage
pub use var_quantity::uom::si::electric_current::{
    ampere, gigaampere, kiloampere, megaampere, microampere, milliampere,
};
pub use var_quantity::uom::si::electric_potential::{
    gigavolt, kilovolt, megavolt, microvolt, millivolt, volt,
};
pub use var_quantity::uom::si::electrical_conductance::{
    gigasiemens, kilosiemens, megasiemens, microsiemens, millisiemens, siemens,
};
pub use var_quantity::uom::si::electrical_conductivity::{
    siemens_per_centimeter, siemens_per_meter,
};
pub use var_quantity::uom::si::electrical_resistance::{
    gigaohm, kiloohm, megaohm, microohm, milliohm, ohm,
};
pub use var_quantity::uom::si::electrical_resistivity::{
    gigaohm_meter, kiloohm_meter, megaohm_meter, microohm_meter, milliohm_meter, ohm_centimeter,
    ohm_meter, ohm_square_millimeter_per_meter,
};

// Magnetism
pub use var_quantity::uom::si::magnetic_field_strength::{
    ampere_per_centimeter, ampere_per_meter, ampere_per_micrometer,
};
pub use var_quantity::uom::si::magnetic_flux::{
    gigaweber, kiloweber, megaweber, microweber, milliweber, weber,
};
pub use var_quantity::uom::si::magnetic_flux_density::{
    gigatesla, kilotesla, megatesla, microtesla, millitesla, tesla,
};
pub use var_quantity::uom::si::magnetic_permeability::henry_per_meter;

// Temperature and heat
pub use var_quantity::uom::si::heat_capacity::{joule_per_degree_celsius, joule_per_kelvin};
pub use var_quantity::uom::si::specific_heat_capacity::{
    joule_per_kilogram_kelvin, kilojoule_per_kilogram_kelvin,
};
pub use var_quantity::uom::si::thermal_conductance::{
    gigawatt_per_kelvin, kilowatt_per_kelvin, megawatt_per_kelvin, microwatt_per_kelvin,
    milliwatt_per_kelvin, watt_per_kelvin,
};
pub use var_quantity::uom::si::thermal_conductivity::{
    gigawatt_per_meter_kelvin, kilowatt_per_meter_kelvin, megawatt_per_meter_kelvin,
    microwatt_per_meter_kelvin, milliwatt_per_meter_kelvin, watt_per_meter_kelvin,
};
pub use var_quantity::uom::si::thermodynamic_temperature::{
    degree_celsius, kelvin, kilokelvin, microkelvin, millikelvin,
};
