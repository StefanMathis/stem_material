/*!
Predefined iron loss models for [`Material`](crate::material::Material)s.

This module contains the [`IronLosses`] enum, a performance optimization of
[`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for iron loss models.
See its docstring for more.

Additionally, it offers the following predefined iron loss models:
- [`JordanModel`] (from submodule [`jordan_model`] )
 */

pub mod jordan_model;
use dyn_quantity::DynQuantity;
pub use jordan_model::*;

use uom::si::f64::SpecificPower;
use var_quantity::{IsQuantityFunction, QuantityFunction};

#[cfg(feature = "serde")]
use serde::Serialize;

/**
A specialized variant of
[`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for iron losses.

In principle, all predefined iron loss models could be treated as
[`IsQuantityFunction`] trait objects, which would allow using [`VarQuantity<SpecificPower>`](var_quantity::VarQuantity) for the
[`Material::iron_losses`](crate::material::Material::iron_losses) field.
However, giving them specific enum variants within [`IronLosses`] improves
performance drastically, since no dynamic dispatch is needed when using these
models. Nevertheless, user-defined iron loss models are still supported via
the [`IronLosses::Function`] variant.
 */
#[derive(Clone, Debug)]
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
    can be realized with the contained [`IsQuantityFunction`] trait object, as
    long as the unit constraint outlined in the [`VarQuantity`] docstring is
    upheld.
     */
    Function(QuantityFunction<SpecificPower>),
}

impl IronLosses {
    /**
    Matches against `self` and calculates the iron losses (or just return the
    value in case of the [`IronLosses::Constant`]) variant).
    */
    pub fn get(&self, conditions: &[DynQuantity<f64>]) -> SpecificPower {
        match self {
            Self::Constant(val) => val.clone(),
            Self::JordanModel(model) => model.call(conditions).try_into().expect("implementation of JordanModel makes sure the returned value is always a SpecificPower"),
            Self::Function(fun) => fun.call(conditions),
        }
    }

    /**
    Returns a reference to the underlying function if `self` is a
    [`IronLosses::Function`].
     */
    pub fn function(&self) -> Option<&dyn IsQuantityFunction> {
        match self {
            Self::Function(quantity_function) => return Some(quantity_function.as_ref()),
            _ => return None,
        }
    }
}

impl TryFrom<Box<dyn IsQuantityFunction>> for IronLosses {
    type Error = dyn_quantity::UnitsNotEqual;

    fn try_from(value: Box<dyn IsQuantityFunction>) -> Result<Self, Self::Error> {
        let wrapper = QuantityFunction::new(value)?;
        return Ok(Self::Function(wrapper));
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
        #[derive(Serialize)]
        enum PredefinedModels<'a> {
            JordanModel(&'a JordanModel),
        }

        #[derive(Serialize)]
        #[serde(untagged)]
        enum IronLossesSerde<'a> {
            Constant(SpecificPower),
            PredefinedModels(PredefinedModels<'a>),
            Function(&'a QuantityFunction<SpecificPower>),
        }

        let il = match self {
            IronLosses::Constant(v) => IronLossesSerde::Constant(*v),
            IronLosses::JordanModel(model) => {
                IronLossesSerde::PredefinedModels(PredefinedModels::JordanModel(model))
            }
            IronLosses::Function(quantity_function) => IronLossesSerde::Function(quantity_function),
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

        use dyn_quantity::DynQuantity;
        use serde::Deserialize;

        #[derive(Deserialize)]
        enum PredefinedModels {
            JordanModel(JordanModel),
        }

        #[derive(deserialize_untagged_verbose_error::DeserializeUntaggedVerboseError)]
        enum IronLossesSerde {
            Constant(SpecificPower),
            String(String),
            PredefinedModels(PredefinedModels),
            Function(QuantityFunction<SpecificPower>),
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
