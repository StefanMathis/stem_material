use stem_material::unary::Linear;
use stem_material::uom::si::f64::MagneticFieldStrength;
use stem_material::uom::si::magnetic_field_strength::ampere_per_meter;
use stem_material::*;

#[test]
fn test_eq() {
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

    assert_eq!(material, material);
    assert_eq!(&material, &material);

    let second_material = Material::default();
    assert_ne!(material, second_material);
    assert_ne!(&material, &second_material);
}
