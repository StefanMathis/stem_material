/*!
Predefined iron loss models for [`Material`](crate::material::Material)s.

This module contains the [`IronLosses`] enum, a performance optimization of
[`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for iron loss models.
See its docstring for more.

Additionally, it offers the following predefined iron loss models:
- [`JordanModel`] (from submodule [`jordan_model`] )
 */

pub mod jordan_model;
pub use jordan_model::*;

use var_quantity::uom::si::f64::SpecificPower;
use var_quantity::{DynQuantity, QuantityFunction, UnitFromType, UnitsNotEqual};

/**
A specialized variant of
[`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for iron losses.

In principle, all predefined iron loss models could be treated as
[`QuantityFunction`] trait objects, which would allow using [`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for the
[`Material::iron_losses`](crate::material::Material::iron_losses) field.
However, giving them specific enum variants within [`IronLosses`] improves
performance drastically, since no dynamic dispatch is needed when using these
models. Nevertheless, user-defined iron loss models are still supported via
the [`IronLosses::Function`] variant.
 */
#[derive(Clone)]
pub enum IronLosses {
    /**
    Optimization for the common case of a constant quantity. This avoids going
    through dynamic dispatch when accessing the value.
     */
    Constant(SpecificPower),
    /**
    Optimization for the common case of using the [`JordanModel`] defined within
    this crate. This avoids going through dynamic dispatch when accessing the
    model.
     */
    JordanModel(JordanModel),
    /**
    Catch-all variant for any non-constant behaviour. Arbitrary behaviour
    can be realized with the contained [`QuantityFunction`] trait object, as
    long as the unit constraint outlined in the
    [`VarQuantity`](var_quantity::VarQuantity) docstring is upheld.
     */
    Function(Box<dyn QuantityFunction>),
}

impl std::fmt::Debug for IronLosses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Constant(arg0) => f.debug_tuple("Constant").field(arg0).finish(),
            Self::JordanModel(arg0) => f.debug_tuple("JordanModel").field(arg0).finish(),
            Self::Function(_) => f.debug_tuple("Function").finish(),
        }
    }
}

impl PartialEq for IronLosses {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Constant(l0), Self::Constant(r0)) => l0 == r0,
            (Self::JordanModel(l0), Self::JordanModel(r0)) => l0 == r0,
            (Self::Function(l0), Self::Function(r0)) => l0.dyn_eq(&**r0),
            _ => false,
        }
    }
}

impl IronLosses {
    /**
    Creates a new [`IronLosses`] instance if the output of the given `function`
    can be converted into [`SpecificPower`].

    As [`IronLosses`] is a specialized variant of
    [`VarQuantity`](var_quantity::VarQuantity), this is the specialized variant
    of [`VarQuantity::new`](var_quantity::VarQuantity::new) which checks if the
    output of `function` can be converted into [`SpecificPower`].

    Due to the additional conversion check, it is recommended to use this
    function instead of directly constructing the [`IronLosses::Function`]
    variant.
    */
    pub fn new<F: QuantityFunction>(function: F) -> Result<Self, UnitsNotEqual> {
        // Check if the function produced the correct type for "default"
        // conditions.
        let output = function.eval(&[]);
        if let Err(_) = SpecificPower::try_from(output) {
            return Err(UnitsNotEqual(SpecificPower::unit_from_type(), output.unit));
        }
        return Ok(Self::Function(Box::new(function)));
    }

    /**
    Matches against `self` and calculates the iron losses (or just return the
    value in case of the [`IronLosses::Constant`]) variant).
    */
    pub fn get(&self, conditions: &[DynQuantity<f64>]) -> SpecificPower {
        match self {
            Self::Constant(val) => val.clone(),
            Self::JordanModel(model) => model.eval(conditions).try_into().expect("implementation of JordanModel makes sure the returned value is always a SpecificPower"),
            Self::Function(fun) => SpecificPower::try_from(fun.eval(conditions)).unwrap_or_default(),
        }
    }

    /**
    Returns a reference to the underlying function if `self` is a
    [`IronLosses::Function`].
     */
    pub fn function(&self) -> Option<&dyn QuantityFunction> {
        match self {
            Self::Function(quantity_function) => return Some(quantity_function.as_ref()),
            _ => return None,
        }
    }
}

impl From<SpecificPower> for IronLosses {
    fn from(value: SpecificPower) -> Self {
        return Self::Constant(value);
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for IronLosses {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::Serialize;
        use var_quantity::serialize_quantity;

        #[derive(Serialize)]
        enum PredefinedModels<'a> {
            JordanModel(&'a JordanModel),
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum IronLossesSerde<'a> {
            #[serde(serialize_with = "serialize_quantity")]
            Constant(SpecificPower),
            PredefinedModels(PredefinedModels<'a>),
            Function(&'a dyn QuantityFunction),
        }

        let il = match self {
            IronLosses::Constant(v) => IronLossesSerde::Constant(*v),
            IronLosses::JordanModel(model) => {
                IronLossesSerde::PredefinedModels(PredefinedModels::JordanModel(model))
            }
            IronLosses::Function(quantity_function) => {
                IronLossesSerde::Function(&**quantity_function)
            }
        };
        il.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IronLosses {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::str::FromStr;

        use serde::Deserialize;
        use var_quantity::DynQuantity;

        #[derive(Deserialize)]
        enum PredefinedModels {
            JordanModel(JordanModel),
        }

        #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
        enum IronLossesSerde {
            Constant(SpecificPower),
            String(String),
            PredefinedModels(PredefinedModels),
            Function(Box<dyn QuantityFunction>),
        }

        let losses_de = IronLossesSerde::deserialize(deserializer)?;
        let losses = match losses_de {
            IronLossesSerde::Constant(v) => IronLosses::Constant(v),
            IronLossesSerde::String(string) => {
                let dyn_quantity =
                    DynQuantity::<f64>::from_str(&string).map_err(serde::de::Error::custom)?;
                let static_quantity =
                    SpecificPower::try_from(dyn_quantity).map_err(serde::de::Error::custom)?;
                IronLosses::Constant(static_quantity)
            }
            IronLossesSerde::PredefinedModels(pd) => match pd {
                PredefinedModels::JordanModel(jordan_model) => {
                    IronLosses::JordanModel(jordan_model)
                }
            },
            IronLossesSerde::Function(quantity_function) => IronLosses::Function(quantity_function),
        };
        return Ok(losses);
    }
}
