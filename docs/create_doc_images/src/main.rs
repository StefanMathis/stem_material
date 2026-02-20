/*!
This crate creates the images used in the documentation of the stem_material crate.
 */

fn main() -> Result<(), Box<dyn std::error::Error>> {}

fn plot_ferromagnetic_permeability() -> Result<(), Box<dyn std::error::Error>> {
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
            .try_into()?;

    return Ok(());
}

fn plot_jordan_model() -> Result<(), Box<dyn std::error::Error>> {
    let iron_loss_data = IronLossData(vec![
        IronLossCharacteristic::new(
            Frequency::new::<hertz>(50.0),
            vec![
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.5),
                    SpecificPower::new::<watt_per_kilogram>(0.4),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.6),
                    SpecificPower::new::<watt_per_kilogram>(0.54),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.7),
                    SpecificPower::new::<watt_per_kilogram>(0.69),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.8),
                    SpecificPower::new::<watt_per_kilogram>(0.86),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.9),
                    SpecificPower::new::<watt_per_kilogram>(1.04),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.0),
                    SpecificPower::new::<watt_per_kilogram>(1.23),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.1),
                    SpecificPower::new::<watt_per_kilogram>(1.44),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.2),
                    SpecificPower::new::<watt_per_kilogram>(1.69),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.3),
                    SpecificPower::new::<watt_per_kilogram>(1.99),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.4),
                    SpecificPower::new::<watt_per_kilogram>(2.37),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.5),
                    SpecificPower::new::<watt_per_kilogram>(2.79),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.6),
                    SpecificPower::new::<watt_per_kilogram>(3.11),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.7),
                    SpecificPower::new::<watt_per_kilogram>(3.38),
                ),
            ],
        ),
        IronLossCharacteristic::new(
            Frequency::new::<hertz>(100.0),
            vec![
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.5),
                    SpecificPower::new::<watt_per_kilogram>(0.84),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.6),
                    SpecificPower::new::<watt_per_kilogram>(1.14),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.7),
                    SpecificPower::new::<watt_per_kilogram>(1.5),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.8),
                    SpecificPower::new::<watt_per_kilogram>(1.88),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.9),
                    SpecificPower::new::<watt_per_kilogram>(2.32),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.0),
                    SpecificPower::new::<watt_per_kilogram>(2.8),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.1),
                    SpecificPower::new::<watt_per_kilogram>(3.33),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.2),
                    SpecificPower::new::<watt_per_kilogram>(3.96),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.3),
                    SpecificPower::new::<watt_per_kilogram>(4.68),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.4),
                    SpecificPower::new::<watt_per_kilogram>(5.58),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.5),
                    SpecificPower::new::<watt_per_kilogram>(6.7),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.6),
                    SpecificPower::new::<watt_per_kilogram>(7.62),
                ),
            ],
        ),
        IronLossCharacteristic::new(
            Frequency::new::<hertz>(200.0),
            vec![
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.5),
                    SpecificPower::new::<watt_per_kilogram>(2.22),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.6),
                    SpecificPower::new::<watt_per_kilogram>(3.07),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.7),
                    SpecificPower::new::<watt_per_kilogram>(4.06),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.8),
                    SpecificPower::new::<watt_per_kilogram>(5.19),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(0.9),
                    SpecificPower::new::<watt_per_kilogram>(6.45),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.0),
                    SpecificPower::new::<watt_per_kilogram>(7.91),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.1),
                    SpecificPower::new::<watt_per_kilogram>(9.53),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.2),
                    SpecificPower::new::<watt_per_kilogram>(11.39),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.3),
                    SpecificPower::new::<watt_per_kilogram>(13.52),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.4),
                    SpecificPower::new::<watt_per_kilogram>(16.37),
                ),
                FluxDensityLossPair::new(
                    MagneticFluxDensity::new::<tesla>(1.5),
                    SpecificPower::new::<watt_per_kilogram>(19.45),
                ),
            ],
        ),
    ]);

    let coeffs = JordanModel::try_from(&iron_loss_data)?;

    return Ok(());
}
