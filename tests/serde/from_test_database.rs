use approx;
use serde_mosaic::{DatabaseManager, SerdeYaml};
use stem_material::*;

fn create_dbm() -> DatabaseManager {
    return DatabaseManager::open("stem_test_database/src", SerdeYaml).expect("must exist");
}

#[test]
fn test_copper() {
    let mut dbm = create_dbm();
    let copper: Material = dbm.read("Copper").unwrap();

    approx::assert_abs_diff_eq!(
        copper
            .electrical_resistivity()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ohm_meter>(),
        1.78571429e-8,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        copper
            .electrical_resistivity()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<ohm_meter>(),
        2.4875e-8,
        epsilon = 0.001
    );

    approx::assert_abs_diff_eq!(
        copper
            .relative_permeability()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]),
        0.9999904,
        epsilon = 0.001
    );

    approx::assert_abs_diff_eq!(
        copper
            .mass_density()
            .get(&[])
            .get::<kilogram_per_cubic_meter>(),
        8920.0,
        epsilon = 0.001
    );
}

#[test]
fn test_ferrite_magnet() {
    let ferrite: Material = create_dbm().read("NMF-12J 430mT").unwrap();

    approx::assert_abs_diff_eq!(
        ferrite
            .electrical_resistivity()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<ohm_meter>(),
        1e6,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        ferrite
            .remanence()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(20.0).into()])
            .get::<tesla>(),
        0.43,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        ferrite
            .remanence()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()])
            .get::<tesla>(),
        0.355481,
        epsilon = 0.001
    );
}

#[test]
fn test_lamination_1() {
    let lamination: Material = create_dbm().read("M270-50A").unwrap();

    // No magnetic flux density
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(0.0).into()]),
        8045.868,
        epsilon = 0.001
    );

    // Temperature argument
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[ThermodynamicTemperature::new::<degree_celsius>(120.0).into()]),
        8045.868,
        epsilon = 0.001
    );

    // Positive and negative flux density
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(1.0).into()]),
        6129.606,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(-1.0).into()]),
        6129.606,
        epsilon = 0.001
    );

    // Extremely high value for the flux density
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(99.0).into()]),
        1.0775785,
        epsilon = 0.001
    );
}

#[test]
fn test_lamination_2() {
    use uom::si::specific_power::watt_per_kilogram;

    let lamination: Material = create_dbm().read("M800-50A").unwrap();

    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(0.5).into()]),
        3801.993,
        epsilon = 0.001
    );
    approx::assert_abs_diff_eq!(
        lamination
            .relative_permeability()
            .get(&[MagneticFluxDensity::new::<tesla>(1.0).into()]),
        3777.110,
        epsilon = 0.001
    );

    // Calculated loss coefficients
    if let VarQuantity::Function(fun) = lamination.iron_losses {
        let model: &JordanModel = (fun.as_ref() as &dyn std::any::Any).downcast_ref().unwrap();
        approx::assert_abs_diff_eq!(
            model.hysteresis_coefficient.get::<watt_per_kilogram>(),
            4.257,
            epsilon = 0.001
        );
        approx::assert_abs_diff_eq!(
            model.eddy_current_coefficient.get::<watt_per_kilogram>(),
            1.262,
            epsilon = 0.001
        );
    } else {
        panic!("should not be a constant")
    }
}

#[test]
fn test_titan() {
    let titan: Material = create_dbm().read("Titan").unwrap();
    approx::assert_abs_diff_eq!(
        1.0 / (2.7e6),
        titan.electrical_resistivity().get(&[]).get::<ohm_meter>(),
        epsilon = 1e-6
    )
}
