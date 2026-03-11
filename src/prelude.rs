/*!
This module reexports all types needed to use stem_material - both those defined
within this crate as well as those in the [si](crate::si) module. The intention
is that one can include use `stem_material::prelude::*` to work efficiently with
this crate.
 */

pub use crate::iron_losses::*;
pub use crate::material::*;
pub use crate::relative_permeability::*;
pub use crate::si::*;

// Reexport var_quantity, which is part of the public API of this crate
pub use var_quantity::*;
