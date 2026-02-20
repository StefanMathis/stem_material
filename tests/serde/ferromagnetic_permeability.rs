use indoc::indoc;
use stem_material::*;
use var_quantity::IsQuantityFunction;

#[test]
fn test_serialize_and_deserialize_relative_permeability() {
    let field_strength: Vec<_> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
    ]
    .into_iter()
    .map(MagneticFieldStrength::new::<ampere_per_meter>)
    .collect();
    let flux_density: Vec<_> = vec![
        0.0, 0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701,
        1.0672, 1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674,
        2.1674, 2.2674, 2.3674, 2.4674, 2.4720,
    ]
    .into_iter()
    .map(MagneticFluxDensity::new::<tesla>)
    .collect();

    let permeability: FerromagneticPermeability =
        MagnetizationCurve::new(field_strength.clone(), flux_density, 0.95)
            .unwrap()
            .try_into()
            .unwrap();

    let serialized = serde_yaml::to_string(&permeability).unwrap();
    let de_permeability: FerromagneticPermeability = serde_yaml::from_str(&serialized).unwrap();

    let conditions = &[MagneticFluxDensity::new::<tesla>(1.5).into()];
    approx::assert_abs_diff_eq!(
        permeability.call(conditions).value,
        de_permeability.call(conditions).value,
        epsilon = 0.001
    );

    let conditions = &[MagneticFluxDensity::new::<tesla>(0.5).into()];
    approx::assert_abs_diff_eq!(
        permeability.call(conditions).value,
        de_permeability.call(conditions).value,
        epsilon = 0.001
    );

    let conditions = &[MagneticFluxDensity::new::<tesla>(-0.5).into()];
    approx::assert_abs_diff_eq!(
        permeability.call(conditions).value,
        de_permeability.call(conditions).value,
        epsilon = 0.001
    );

    let conditions = &[MagneticFluxDensity::new::<tesla>(-10.0).into()];
    approx::assert_abs_diff_eq!(
        permeability.call(conditions).value,
        de_permeability.call(conditions).value,
        epsilon = 0.001
    );
}

#[test]
fn test_deserialize_relative_permeability_from_raw_data() {
    let serialized = indoc! {"
    field_strength: '[
          0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
          276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
          69372.42, 102918.79, 150142.01, 215692.99
        ] A/m'
    flux_density: '[
            0.0, 0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701,
            1.0672, 1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674,
            2.1674, 2.2674, 2.3674, 2.4674
        ] T'
    iron_fill_factor: 0.95
    "};

    let de_permeability: FerromagneticPermeability = serde_yaml::from_str(&serialized).unwrap();

    approx::assert_abs_diff_eq!(
        de_permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-0.5).into()])
            .value,
        8045.868,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        de_permeability
            .call(&[MagneticFluxDensity::new::<tesla>(10.0).into()])
            .value,
        8.2107,
        epsilon = 0.001
    );
}
