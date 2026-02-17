#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod ferromagnetic_permeability;
mod jordan_model;
mod material;

pub use ferromagnetic_permeability::*;
pub use jordan_model::*;
pub use material::*;
pub use var_quantity::*;
