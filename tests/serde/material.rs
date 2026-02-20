use dyn_quantity::{DynQuantity, PredefUnit, Unit};
use indoc::indoc;
use stem_material::*;
use uom::si::magnetic_flux_density::tesla;
use uom::si::specific_power::watt_per_kilogram;
use var_quantity::{VarQuantity, unary::Linear};

#[test]
fn test_serialize_material() {
    let mut material = Material::default();

    let linear = Linear::new(
        DynQuantity::new(
            2.0,
            Unit::from(PredefUnit::MagneticFluxDensity) / Unit::from(PredefUnit::Temperature),
        ),
        DynQuantity::new(1.0, PredefUnit::MagneticFluxDensity),
    );
    material.set_remanence(VarQuantity::try_from_quantity_function(linear).unwrap());
    material.set_intrinsic_coercivity(VarQuantity::Constant(MagneticFieldStrength::new::<
        ampere_per_meter,
    >(5.0)));

    let string = serde_yaml::to_string(&material).unwrap();
    let material: Material = serde_yaml::from_str(&string).unwrap();

    let conditions = [ThermodynamicTemperature::new::<degree_celsius>(20.0).into()];

    assert_eq!(material.remanence().get(&conditions).get::<tesla>(), 587.3);
    assert_eq!(
        material
            .intrinsic_coercivity()
            .get(&conditions)
            .get::<ampere_per_meter>(),
        5.0
    );
}

#[test]
fn test_deserialize_material() {
    // Property thermal_conductivity is purposefully missing
    let serialized = indoc! {"
    ---
    name: M800-50A
    relative_permeability:
      FerromagneticPermeability:
        field_strength: '[
              0.0, 130.0, 141.0, 153.0, 166.0, 181.0, 198.0, 221.0, 252.0, 304.0, 409.0, 680.0, 1540.0,
              3789.0, 7752.0, 13730.0
              ] A/m'
        flux_density: '[
              0.0, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9
              ] T'
        iron_fill_factor: 0.95
    remanence: 0.0 T
    iron_losses:
      JordanModel:
        - frequency: 50.0 Hz
          characteristic:
            - flux_density: 0.5 T
              specific_loss: 0.86 W/kg
            - flux_density: 0.6 T
              specific_loss: 1.16 W/kg
            - flux_density: 0.7 T
              specific_loss: 1.47 W/kg
            - flux_density: 0.8 T
              specific_loss: 1.82 W/kg
            - flux_density: 0.9 T
              specific_loss: 2.2 W/kg
            - flux_density: 1.0 T
              specific_loss: 2.6 W/kg
            - flux_density: 1.1 T
              specific_loss: 3.06 W/kg
            - flux_density: 1.2 T
              specific_loss: 3.57 W/kg
            - flux_density: 1.3 T
              specific_loss: 4.14 W/kg
            - flux_density: 1.4 T
              specific_loss: 4.79 W/kg
            - flux_density: 1.5 T
              specific_loss: 5.52 W/kg
            - flux_density: 1.6 T
              specific_loss: 6.37 W/kg
            - flux_density: 1.7 T
              specific_loss: 7.08 W/kg
            - flux_density: 1.8 T
              specific_loss: 7.65 W/kg
            - flux_density: 1.9 T
              specific_loss: 8.12 W/kg
        - frequency: 100.0 Hz
          characteristic:
            - flux_density: 0.5 T
              specific_loss: 1.93 W/kg
            - flux_density: 0.6 T
              specific_loss: 2.62 W/kg
            - flux_density: 0.7 T
              specific_loss: 3.38 W/kg
            - flux_density: 0.8 T
              specific_loss: 4.22 W/kg
            - flux_density: 0.9 T
              specific_loss: 5.15 W/kg
            - flux_density: 1.0 T
              specific_loss: 6.19 W/kg
            - flux_density: 1.1 T
              specific_loss: 7.34 W/kg
            - flux_density: 1.2 T
              specific_loss: 8.65 W/kg
            - flux_density: 1.3 T
              specific_loss: 10.11 W/kg
            - flux_density: 1.4 T
              specific_loss: 11.74 W/kg
            - flux_density: 1.5 T
              specific_loss: 13.56 W/kg
        - frequency: 200.0 Hz
          characteristic:
            - flux_density: 0.5 T
              specific_loss: 4.63 W/kg
            - flux_density: 0.6 T
              specific_loss: 6.37 W/kg
            - flux_density: 0.7 T
              specific_loss: 8.35 W/kg
            - flux_density: 0.8 T
              specific_loss: 10.59 W/kg
            - flux_density: 0.9 T
              specific_loss: 13.2 W/kg
            - flux_density: 1.0 T
              specific_loss: 16.15 W/kg
            - flux_density: 1.1 T
              specific_loss: 19.31 W/kg
            - flux_density: 1.2 T
              specific_loss: 23.08 W/kg
            - flux_density: 1.3 T
              specific_loss: 27.24 W/kg
            - flux_density: 1.4 T
              specific_loss: 32.42 W/kg
            - flux_density: 1.5 T
              specific_loss: 37.56 W/kg
    intrinsic_coercivity: 5.0 A/m
    mass_density: 7650.0 kg / m^3
    electrical_resistivity:
      FirstOrderTaylor:
        base_value: 1 / 56 m/MS
        expansion_point: 20 °C
        slope: 0.393 % / K
    heat_capacity: 435.0 J / kg / K
    "};
    let material: Material = serde_yaml::from_str(&serialized).unwrap();

    let conditions = &[MagneticFluxDensity::new::<tesla>(0.5).into()];
    approx::assert_abs_diff_eq!(
        material.relative_permeability().get(conditions),
        3801.0,
        epsilon = 0.1
    );
    approx::assert_abs_diff_eq!(
        material
            .mass_density()
            .get(conditions)
            .get::<kilogram_per_cubic_meter>(),
        7650.0
    );

    approx::assert_abs_diff_eq!(
        material
            .electrical_resistivity()
            .get(conditions)
            .get::<ohm_meter>(),
        1.7857e-8,
        epsilon = 1e-12
    );

    let conditions = &[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()];
    approx::assert_abs_diff_eq!(
        material
            .electrical_resistivity()
            .get(conditions)
            .get::<ohm_meter>(),
        2.4875e-8,
        epsilon = 1e-12
    );

    if let RelativePermeability::FerromagneticPermeability(model) = &material.relative_permeability
    {
        approx::assert_abs_diff_eq!(
            model.get(MagneticFluxDensity::new::<tesla>(0.5)),
            3801.0,
            epsilon = 0.1
        );
    } else {
        panic!("wrong model");
    }

    if let IronLosses::JordanModel(model) = &material.iron_losses {
        approx::assert_abs_diff_eq!(
            model.eddy_current_coefficient.get::<watt_per_kilogram>(),
            1.2615,
            epsilon = 0.001
        );
        approx::assert_abs_diff_eq!(
            model.hysteresis_coefficient.get::<watt_per_kilogram>(),
            4.2568,
            epsilon = 0.001
        );
    } else {
        panic!("wrong model");
    }
}
