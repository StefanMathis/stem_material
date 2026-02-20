use stem_material::relative_permeability::*;
use uom::si::f64::*;
use uom::si::magnetic_field_strength::ampere_per_meter;
use uom::si::magnetic_flux_density::tesla;
use var_quantity::IsQuantityFunction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    for flux_density in [0.1, -0.1].into_iter() {
        approx::assert_abs_diff_eq!(
            permeability
                .call(&[MagneticFluxDensity::new::<tesla>(flux_density).into()])
                .value,
            8469.282,
            epsilon = 0.001
        );
    }
    for flux_density in [1.5, -1.5].into_iter() {
        approx::assert_abs_diff_eq!(
            permeability
                .call(&[MagneticFluxDensity::new::<tesla>(flux_density).into()])
                .value,
            503.64,
            epsilon = 0.001
        );
    }
    for flux_density in [2.5, -2.5].into_iter() {
        approx::assert_abs_diff_eq!(
            permeability
                .call(&[MagneticFluxDensity::new::<tesla>(flux_density).into()])
                .value,
            9.048,
            epsilon = 0.001
        );
    }
    return Ok(());
}
