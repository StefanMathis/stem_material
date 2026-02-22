/*!
This crate creates the images used in the documentation of the stem_material crate.
 */
use plotters::element::DashedPathElement;
use plotters::prelude::*;
use plotters::style::full_palette::GREEN_800;
use stem_material::uom::si::f64::*;
use stem_material::uom::si::specific_power::watt_per_kilogram;
use stem_material::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    plot_ferromagnetic_permeability()?;
    plot_jordan_model()?;
    return Ok(());
}

fn plot_ferromagnetic_permeability() -> Result<(), Box<dyn std::error::Error>> {
    let field_strength = vec![
        11.57, 22.11, 31.71, 40.47, 48.50, 55.29, 64.02, 75.66, 89.24, 107.67, 134.83, 179.45,
        276.45, 582.98, 1583.11, 3578.65, 6665.91, 11303.32, 18871.00, 29765.16, 45905.16,
        69372.42, 102918.79, 150142.01, 215692.99, 219224.15,
    ];

    let fs: Vec<_> = field_strength
        .iter()
        .cloned()
        .map(MagneticFieldStrength::new::<ampere_per_meter>)
        .collect();

    let flux_density = vec![
        0.0970, 0.1940, 0.2910, 0.3880, 0.4851, 0.5821, 0.6791, 0.7761, 0.8731, 0.9701, 1.0672,
        1.1642, 1.2614, 1.3588, 1.4571, 1.5566, 1.6576, 1.7606, 1.8674, 1.9674, 2.0674, 2.1674,
        2.2674, 2.3674, 2.4674, 2.4720,
    ];
    let fd: Vec<_> = flux_density
        .iter()
        .cloned()
        .map(MagneticFluxDensity::new::<tesla>)
        .collect();

    let fp_100: FerromagneticPermeability = MagnetizationCurve::new(fs.clone(), fd.clone(), 1.0)
        .unwrap()
        .try_into()?;

    let fp_95: FerromagneticPermeability =
        MagnetizationCurve::new(fs, fd, 0.95).unwrap().try_into()?;

    // General config
    // =========================================================================
    let size = (600, 400);
    let font_size_labels = 18;
    let font_size_ticks = 16;
    let font_size_legend = 18;

    // Plot the individual data points and the lines calculated from jordan_model
    // =========================================================================

    let file_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&"../relative_permeability.svg");
    let root = SVGBackend::new(&file_path, size).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(80)
        .build_cartesian_2d(0.0..3.0, 0.0..9000.0)?;

    chart
        .configure_mesh()
        .x_desc("Flux density in T")
        .y_desc("Relative permeability")
        .axis_desc_style(("sans-serif", font_size_labels))
        .label_style(("sans-serif", font_size_ticks))
        .draw()?;

    chart
        .draw_series(
            field_strength
                .iter()
                .zip(flux_density.iter())
                .map(move |(fs, fd)| {
                    let rp = *fd / (*fs * VACUUM_PERMEABILITY_UNITLESS);
                    Cross::new((*fd, rp), 5.0, BLUE.filled())
                }),
        )?
        .label("raw data")
        .legend(move |(x, y)| Cross::new((x + 10, y), 5.0, BLUE.filled()));

    let mut fd_vec = Vec::new();
    let mut fd = 0.0;
    while fd <= 3.0 {
        fd_vec.push(fd);
        fd += 0.001;
    }

    chart
        .draw_series(LineSeries::new(
            fd_vec
                .iter()
                .cloned()
                .map(|fd| (fd, fp_100.get(MagneticFluxDensity::new::<tesla>(fd)))),
            GREEN_800.stroke_width(1),
        ))?
        .label(&format!("interpolation (iron fill factor = 100 %)"))
        .legend(move |(x, y)| {
            PathElement::new(vec![(x, y), (x + 20, y)], GREEN_800.stroke_width(1))
        });

    chart
        .draw_series(DashedLineSeries::new(
            fd_vec
                .iter()
                .map(|fd| (*fd, fp_95.get(MagneticFluxDensity::new::<tesla>(*fd)))),
            12,
            6,
            GREEN_800.stroke_width(1),
        ))?
        .label("interpolation (iron fill factor = 95 %)")
        .legend(|(x, y)| {
            DashedPathElement::new(vec![(x, y), (x + 20, y)], 8, 4, GREEN_800.stroke_width(1))
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8)) // semi-transparent background
        .label_font(("sans-serif", font_size_legend))
        .position(SeriesLabelPosition::UpperRight) // position on the chart
        .draw()?;

    root.present().expect(&format!(
        "Unable to write result to file, please make sure you have write permissions for {}",
        file_path.display()
    ));

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

    let jordan_model = JordanModel::try_from(&iron_loss_data)?;

    // General config
    // =========================================================================
    let size = (600, 400);
    let font_size_labels = 18;
    let font_size_ticks = 16;
    let font_size_legend = 18;

    // Plot the individual data points and the lines calculated from jordan_model
    // =========================================================================

    let file_path =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&"../jordan_model.svg");
    let root = SVGBackend::new(&file_path, size).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(0.4..1.8, 0.0..20.0)?;

    chart
        .configure_mesh()
        .x_desc("Flux density in T")
        .y_desc("Specific losses in W/kg")
        .axis_desc_style(("sans-serif", font_size_labels))
        .label_style(("sans-serif", font_size_ticks))
        .draw()?;

    // Individal data points
    let colors = [BLUE, GREEN_800, RED];

    for (c, color) in iron_loss_data.0.iter().zip(colors.clone().into_iter()) {
        let f = c.frequency.get::<hertz>();
        chart
            .draw_series(c.characteristic.iter().map(move |pt| {
                Cross::new(
                    (
                        pt.flux_density.get::<tesla>(),
                        pt.specific_loss.get::<watt_per_kilogram>(),
                    ),
                    5.0,
                    color.clone().filled(),
                )
            }))?
            .label(&format!("loss data @ {f} Hz"))
            .legend(move |(x, y)| Cross::new((x + 10, y), 5.0, color.filled()));

        let mut fd_vec = Vec::new();
        let mut l_vec = Vec::new();
        let mut fd = 0.4;
        while fd < 1.8 {
            fd_vec.push(fd);
            l_vec.push(
                jordan_model
                    .losses(MagneticFluxDensity::new::<tesla>(fd), c.frequency)
                    .get::<watt_per_kilogram>(),
            );
            fd += 0.01;
        }

        chart
            .draw_series(LineSeries::new(
                fd_vec.into_iter().zip(l_vec.into_iter()),
                &color,
            ))?
            .label(&format!("model @ {f} Hz"))
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8)) // semi-transparent background
        .label_font(("sans-serif", font_size_legend))
        .position(SeriesLabelPosition::UpperLeft) // position on the chart
        .draw()?;

    root.present().expect(&format!(
        "Unable to write result to file, please make sure you have write permissions for {}",
        file_path.display()
    ));

    return Ok(());
}
