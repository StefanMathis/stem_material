use stem_material::*;

#[test]
fn test_relative_permeability() {
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
        MagnetizationCurve::new(field_strength.clone(), flux_density, 1.0)
            .unwrap()
            .try_into()
            .unwrap();

    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(0.5).into()])
            .value,
        8469.282,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(0.9).into()])
            .value,
        7647.7276,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(1.0).into()])
            .value,
        6924.8432,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(10.0).into()])
            .value,
        8.4290,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(90.0).into()])
            .value,
        1.8254,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(100.0).into()])
            .value,
        1.0,
        epsilon = 0.001
    );

    // Negative flux densities
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-0.5).into()])
            .value,
        8469.282,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-10.0).into()])
            .value,
        8.4290,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-100.0).into()])
            .value,
        1.0,
        epsilon = 0.001
    );
}

#[test]
fn test_relative_permeability_iron_fill_factor() {
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

    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(0.5).into()])
            .value,
        8045.868,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(0.9).into()])
            .value,
        6974.4999,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(1.0).into()])
            .value,
        6129.6062,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(10.0).into()])
            .value,
        8.0496,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(90.0).into()])
            .value,
        1.7833,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(100.0).into()])
            .value,
        1.0,
        epsilon = 0.001
    );

    // Negative flux densities
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(0.5).into()])
            .value,
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-0.5).into()])
            .value,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(10.0).into()])
            .value,
        permeability
            .call(&[MagneticFluxDensity::new::<tesla>(-10.0).into()])
            .value,
        epsilon = 0.001
    );
}

#[test]
fn test_permeability_curve_without_iron_fill_factor() {
    // Construct an ferromagnetic material
    // flux_density(field_strength) curve (not considering hysteresis, not
    // homogenized). Source: M270-50A_nicht_homogenisiert.tab
    let field_strength: Vec<MagneticFieldStrength> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
    ]
    .into_iter()
    .map(MagneticFieldStrength::new::<ampere_per_meter>)
    .collect();

    let mut flux_density: Vec<MagneticFluxDensity> = vec![
        0.0, 0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701,
        1.0672, 1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674,
        2.1674, 2.2674, 2.3674, 2.4674, 2.4720,
    ]
    .into_iter()
    .map(MagneticFluxDensity::new::<tesla>)
    .collect();

    for (bi, hi) in flux_density.iter_mut().zip(field_strength.iter()) {
        *bi = *bi * 0.95 + (1.0 - 0.95) * *hi * *VACUUM_PERMEABILITY;
    }

    // Create permeability characteristic
    let fp = FerromagneticPermeability::from_magnetization(
        MagnetizationCurve::new(field_strength.clone(), flux_density.clone(), 1.0).unwrap(),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(0.5).unwrap(),
        8045.868,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(0.9).unwrap(),
        6974.4999,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(1.0).unwrap(),
        6129.6062,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(10.0).unwrap(),
        8.0057,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(90.0).unwrap(),
        1.7784,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(100.0).unwrap(),
        1.0,
        epsilon = 0.001
    );

    // Recreate the B(H) curve from the permeability curve
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[1].get::<ampere_per_meter>())
            .unwrap(),
        8045.868,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[5].get::<ampere_per_meter>())
            .unwrap(),
        8045.868,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[10].get::<ampere_per_meter>())
            .unwrap(),
        flux_density[10].get::<tesla>()
            / VACUUM_PERMEABILITY_UNITLESS
            / field_strength[10].get::<ampere_per_meter>(),
        epsilon = 1.0
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[15].get::<ampere_per_meter>())
            .unwrap(),
        flux_density[15].get::<tesla>()
            / VACUUM_PERMEABILITY_UNITLESS
            / field_strength[15].get::<ampere_per_meter>(),
        epsilon = 0.02
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[20].get::<ampere_per_meter>())
            .unwrap(),
        flux_density[20].get::<tesla>()
            / VACUUM_PERMEABILITY_UNITLESS
            / field_strength[20].get::<ampere_per_meter>(),
        epsilon = 0.02
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[26].get::<ampere_per_meter>())
            .unwrap(),
        8.6002,
        epsilon = 0.02
    );
}

#[test]
fn test_monotonic_decreasing() {
    let field_strength: Vec<MagneticFieldStrength> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
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

    let fp = FerromagneticPermeability::from_magnetization(
        MagnetizationCurve::new(field_strength.clone(), flux_density.clone(), 1.0).unwrap(),
    )
    .unwrap();

    // Check the mu(H) curve
    let mut permeability = 10000.0;
    for idx in 0..300 {
        let field_strength = idx as f64 * 100.0;
        let mu_eval = fp.from_field_strength.eval(field_strength).unwrap();
        assert!(mu_eval <= permeability);
        permeability = mu_eval;
    }

    // Check the mu(B) curve
    let mut permeability = 10000.0;
    for idx in 0..300 {
        let flux_density = idx as f64 / 100.0;
        let mu_eval = fp.from_flux_density.eval(flux_density).unwrap();
        assert!(mu_eval <= permeability);
        permeability = mu_eval;
    }
}

#[test]
fn test_bh_curve_reconstruction() {
    let field_strength: Vec<MagneticFieldStrength> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
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

    let fp = FerromagneticPermeability::from_magnetization(
        MagnetizationCurve::new(field_strength.clone(), flux_density.clone(), 1.0).unwrap(),
    )
    .unwrap();

    // Check some values from the B(H) curve given above
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[10].get::<ampere_per_meter>())
            .unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength[10].get::<ampere_per_meter>(),
        flux_density[10].get::<tesla>(),
        epsilon = 0.02
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[20].get::<ampere_per_meter>())
            .unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength[20].get::<ampere_per_meter>(),
        flux_density[20].get::<tesla>(),
        epsilon = 0.02
    );

    // Recreate the B(H) curve from the mu(H) curve and check if it is strictly
    // monotonically increasing
    let mut flux_density = 0.0;
    for idx in 0..300 {
        let field_strength = idx as f64 * 10.0;
        let b_eval = fp.from_field_strength.eval(field_strength).unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength;
        assert!(b_eval >= flux_density);
        flux_density = b_eval;
    }
}

#[test]
fn test_bh_curve_reconstruction_from_polarization() {
    let field_strength: Vec<MagneticFieldStrength> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
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

    // Convert to polarization
    let polarization: Vec<MagneticFluxDensity> = flux_density
        .iter()
        .zip(field_strength.iter())
        .map(|(b, h)| {
            return *b - *h * *VACUUM_PERMEABILITY;
        })
        .collect();

    let fp = FerromagneticPermeability::from_polarization(
        PolarizationCurve::new(field_strength.clone(), polarization, 1.0).unwrap(),
    )
    .unwrap();

    // Check some values from the B(H) curve given above
    approx::assert_abs_diff_eq!(
        fp.from_flux_density
            .eval(flux_density[10].get::<tesla>())
            .unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength[10].get::<ampere_per_meter>(),
        flux_density[10].get::<tesla>(),
        epsilon = 0.02
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[10].get::<ampere_per_meter>())
            .unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength[10].get::<ampere_per_meter>(),
        flux_density[10].get::<tesla>(),
        epsilon = 0.02
    );
    approx::assert_abs_diff_eq!(
        fp.from_field_strength
            .eval(field_strength[20].get::<ampere_per_meter>())
            .unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength[20].get::<ampere_per_meter>(),
        flux_density[20].get::<tesla>(),
        epsilon = 0.02
    );

    // Recreate the B(H) curve from the mu(H) curve and check if it is strictly
    // monotonically increasing
    let mut flux_density = 0.0;
    for idx in 0..300 {
        let field_strength = idx as f64 * 10.0;
        let b_eval = fp.from_field_strength.eval(field_strength).unwrap()
            * VACUUM_PERMEABILITY_UNITLESS
            * field_strength;
        assert!(b_eval >= flux_density);
        flux_density = b_eval;
    }
}

#[test]
fn test_permeability_curve_with_iron_fill_factor() {
    // Construct an ferromagnetic material
    // flux_density(field_strength) curve (not considering hysteresis, not
    // homogenized). Source: M270-50A_nicht_homogenisiert.tab
    let field_strength: Vec<MagneticFieldStrength> = vec![
        0.0, 11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
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

    // Create permeability characteristic
    let fp = FerromagneticPermeability::from_magnetization(
        MagnetizationCurve::new(field_strength, flux_density, 0.95).unwrap(),
    )
    .unwrap();

    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(1.0).unwrap(),
        6129.606,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(10.0).unwrap(),
        8.049,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        fp.from_flux_density.eval(90.0).unwrap(),
        1.783,
        epsilon = 0.001
    );
}
