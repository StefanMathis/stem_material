use indoc::indoc;
use stem_material::*;
use uom::si::specific_power::watt_per_kilogram;

#[test]
fn test_serialize_and_deserialize_iron_losses() {
    let iron_loss_coeffs = JordanModel::new(
        SpecificPower::new::<watt_per_kilogram>(1.0),
        SpecificPower::new::<watt_per_kilogram>(0.5),
    );

    let serialized = serde_yaml::to_string(&iron_loss_coeffs).unwrap();
    let de_iron_loss_coeffs: JordanModel = serde_yaml::from_str(&serialized).unwrap();

    approx::assert_abs_diff_eq!(
        iron_loss_coeffs
            .eddy_current_coefficient
            .get::<watt_per_kilogram>(),
        de_iron_loss_coeffs
            .eddy_current_coefficient
            .get::<watt_per_kilogram>(),
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        iron_loss_coeffs
            .hysteresis_coefficient
            .get::<watt_per_kilogram>(),
        de_iron_loss_coeffs
            .hysteresis_coefficient
            .get::<watt_per_kilogram>(),
        epsilon = 0.001
    );
}

#[test]
fn test_deserialize_iron_losses() {
    let serialized = indoc! {"
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
    "};

    let de_iron_loss_coeffs: JordanModel = serde_yaml::from_str(&serialized).unwrap();

    approx::assert_abs_diff_eq!(
        de_iron_loss_coeffs
            .eddy_current_coefficient
            .get::<watt_per_kilogram>(),
        1.2615,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        de_iron_loss_coeffs
            .hysteresis_coefficient
            .get::<watt_per_kilogram>(),
        4.2568,
        epsilon = 0.001
    );
}

#[test]
fn test_serialize_and_deserialize_material() {
    let mut material = Material::default();

    // Adjust some properties
    {
        material.set_mass_density(VarQuantity::Constant(MassDensity::new::<
            kilogram_per_cubic_meter,
        >(5.0)));

        let field_strength = vec![
            0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83,
            179.45, 276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16,
            45905.16, 69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
        ]
        .into_iter()
        .map(MagneticFieldStrength::new::<ampere_per_meter>)
        .collect();
        let flux_density = vec![
            0.0, 0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701,
            1.0672, 1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674,
            2.1674, 2.2674, 2.3674, 2.4674, 2.4720,
        ]
        .into_iter()
        .map(MagneticFluxDensity::new::<tesla>)
        .collect();

        let permeability: FerromagneticPermeability =
            MagnetizationCurve::new(field_strength, flux_density, 0.95)
                .unwrap()
                .try_into()
                .unwrap();
        material.set_relative_permeability(
            VarQuantity::try_from_quantity_function(permeability).unwrap(),
        );
    }

    let serialized = serde_yaml::to_string(&material).unwrap();
    let de_material: Material = serde_yaml::from_str(&serialized).unwrap();

    let conditions = &[MagneticFluxDensity::new::<tesla>(0.5).into()];
    approx::assert_abs_diff_eq!(
        material.relative_permeability().get(conditions),
        8045.86,
        epsilon = 0.01
    );
    approx::assert_abs_diff_eq!(
        material.relative_permeability().get(conditions),
        de_material.relative_permeability().get(conditions),
        epsilon = 0.01
    );

    approx::assert_abs_diff_eq!(
        material
            .mass_density()
            .get(conditions)
            .get::<kilogram_per_cubic_meter>(),
        5.0
    );
    approx::assert_abs_diff_eq!(
        material
            .mass_density()
            .get(conditions)
            .get::<kilogram_per_cubic_meter>(),
        de_material
            .mass_density()
            .get(conditions)
            .get::<kilogram_per_cubic_meter>(),
        epsilon = 0.01
    );
}

#[test]
fn test_deserialize_material_only_iron_losses() {
    {
        let serialized = indoc! {"
        ---
        name: M800-50A
        iron_losses:
          JordanModel:
            hysteresis_coefficient: 0.2
            eddy_current_coefficient: 1.0
        "};
        let material: Material = serde_yaml::from_str(&serialized).unwrap();
        if let VarQuantity::Function(fun) = material.iron_losses {
            let model: &JordanModel = (fun.as_ref() as &dyn std::any::Any).downcast_ref().unwrap();
            assert_eq!(
                model.hysteresis_coefficient,
                SpecificPower::new::<watt_per_kilogram>(0.2)
            );
            assert_eq!(
                model.eddy_current_coefficient,
                SpecificPower::new::<watt_per_kilogram>(1.0)
            );
        } else {
            panic!("should not be a constant")
        }
    }
    {
        let serialized = indoc! {"
        ---
        name: M800-50A
        iron_losses:
          JordanModel:
            hysteresis_coefficient: 200 mW / kg
            eddy_current_coefficient: 1000 mW / kg
        "};
        let material: Material = serde_yaml::from_str(&serialized).unwrap();
        if let VarQuantity::Function(fun) = material.iron_losses {
            let model: &JordanModel = (fun.as_ref() as &dyn std::any::Any).downcast_ref().unwrap();
            assert_eq!(
                model.hysteresis_coefficient,
                SpecificPower::new::<watt_per_kilogram>(0.2)
            );
            assert_eq!(
                model.eddy_current_coefficient,
                SpecificPower::new::<watt_per_kilogram>(1.0)
            );
        } else {
            panic!("should not be a constant")
        }
    }
}

#[test]
fn test_deserialize_material_const_permeability() {
    let serialized = indoc! {"
    ---
    name: M800-50A
    relative_permeability: 42.0
    "};
    let material: Material = serde_yaml::from_str(&serialized).unwrap();
    assert_eq!(material.relative_permeability().get(&[]), 42.0);
}
