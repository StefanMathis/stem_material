#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod iron_losses;
pub mod material;
pub mod relative_permeability;

pub use iron_losses::*;
pub use material::*;
pub use relative_permeability::*;

// Reexport var_quantity, which is part of the public API of this crate
pub use var_quantity::*;
