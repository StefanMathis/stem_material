use stem_material::jordan_model::*;
use uom::si::f64::*;
use uom::si::frequency::hertz;
use uom::si::magnetic_flux_density::tesla;
use uom::si::specific_power::watt_per_kilogram;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    Creates `IronLossCharacteristic`s from individual `FluxDensityLossPair`
    value data pairs. Multiple characteristics are concatenated into
    `IronLossData`, which is then used to derive a Jordan model from it.
    Lastly, the loss coefficients of the model are compared to expected values.
     */
    from_flux_density_loss_pair()?;

    /*
    Creates `IronLossCharacteristic`s from vectors of flux densities and losses.
    Multiple characteristics are concatenated into `IronLossData`, which is then
    used to derive a Jordan model from it. Lastly, the loss coefficients of the
    model are compared to expected values.
     */
    from_vecs()?;

    return Ok(());
}

/**
Creates `IronLossCharacteristic`s from individual `FluxDensityLossPair`
value data pairs. Multiple characteristics are concatenated into
`IronLossData`, which is then used to derive a Jordan model from it.
Lastly, the loss coefficients of the model are compared to expected values.
    */
fn from_flux_density_loss_pair() -> Result<(), Box<dyn std::error::Error>> {
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
    approx::assert_abs_diff_eq!(
        coeffs.hysteresis_coefficient.get::<watt_per_kilogram>(),
        2.109,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        coeffs.eddy_current_coefficient.get::<watt_per_kilogram>(),
        0.598,
        epsilon = 0.001
    );
    return Ok(());
}

/**
Creates `IronLossCharacteristic`s from vectors of flux densities and losses.
Multiple characteristics are concatenated into `IronLossData`, which is then
used to derive a Jordan model from it. Lastly, the loss coefficients of the
model are compared to expected values.
    */
fn from_vecs() -> Result<(), Box<dyn std::error::Error>> {
    let iron_loss_data = IronLossData(vec![
        IronLossCharacteristic::from_vecs(
            Frequency::new::<hertz>(50.0),
            &(vec![
                0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9,
            ]
            .into_iter()
            .map(MagneticFluxDensity::new::<tesla>)
            .collect::<Vec<_>>()),
            &(vec![
                0.86, 1.16, 1.47, 1.82, 2.20, 2.60, 3.06, 3.57, 4.14, 4.79, 5.52, 6.37, 7.08, 7.65,
                8.12,
            ]
            .into_iter()
            .map(SpecificPower::new::<watt_per_kilogram>)
            .collect::<Vec<_>>()),
        ),
        IronLossCharacteristic::from_vecs(
            Frequency::new::<hertz>(100.0),
            &(vec![0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5]
                .into_iter()
                .map(MagneticFluxDensity::new::<tesla>)
                .collect::<Vec<_>>()),
            &(vec![
                1.93, 2.62, 3.38, 4.22, 5.15, 6.19, 7.34, 8.65, 10.11, 11.74, 13.56,
            ]
            .into_iter()
            .map(SpecificPower::new::<watt_per_kilogram>)
            .collect::<Vec<_>>()),
        ),
        IronLossCharacteristic::from_vecs(
            Frequency::new::<hertz>(200.0),
            &(vec![0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5]
                .into_iter()
                .map(MagneticFluxDensity::new::<tesla>)
                .collect::<Vec<_>>()),
            &(vec![
                4.63, 6.37, 8.35, 10.59, 13.20, 16.15, 19.31, 23.08, 27.24, 32.42, 37.56,
            ]
            .into_iter()
            .map(SpecificPower::new::<watt_per_kilogram>)
            .collect::<Vec<_>>()),
        ),
    ]);

    let coeffs = JordanModel::try_from(&iron_loss_data)?;
    approx::assert_abs_diff_eq!(
        coeffs.hysteresis_coefficient.get::<watt_per_kilogram>(),
        4.257,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        coeffs.eddy_current_coefficient.get::<watt_per_kilogram>(),
        1.262,
        epsilon = 0.001
    );
    return Ok(());
}
