/*!
[`Material`]: crate::material::Material
[`FerromagneticPermeability`]: crate::relative_permeability::FerromagneticPermeability
[`JordanModel`]: crate::iron_losses::jordan_model::JordanModel
[module]: crate::iron_losses::jordan_model
[var_quantity]: var_quantity

Material definition for stem - a Simulation Toolbox for Electric Motors.

 */
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("relative_permeability.svg", "docs/img/relative_permeability.svg"),
doc = ::embed_doc_image::embed_image!("jordan_model.svg", "docs/img/jordan_model.svg"),
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
#![doc = include_str!("../docs/main.md")]
#![deny(missing_docs)]

pub mod iron_losses;
pub mod material;
pub mod prelude;
pub mod relative_permeability;
pub mod si;

pub use var_quantity;
pub use var_quantity::uom;
