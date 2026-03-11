/*!
[`Material`]: crate::material::Material
[`FerromagneticPermeability`]: crate::relative_permeability::FerromagneticPermeability
[`JordanModel`]: crate::iron_losses::jordan_model::JordanModel
[module]: crate::iron_losses::jordan_model
[var_quantity]: var_quantity
 */
#![doc = include_str!("../docs/readme_parts/relative_permeability.svg.md")]
#![doc = r#"

![Relative permeability][relative_permeability]

"#]
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("relative_permeability", "docs/img/relative_permeability.svg"),
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
#![doc = include_str!("../docs/readme_parts/jordan_model.svg.md")]
#![doc = r#"

![Jordan model][jordan_model]

"#]
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("jordan_model", "docs/img/jordan_model.svg"),
))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile docs with `cargo doc --features 'doc-images'` and Rust version >= 1.54."
)]
#![doc = include_str!("../docs/readme_parts/end.md")]
#![deny(missing_docs)]

pub mod iron_losses;
pub mod material;
pub mod prelude;
pub mod relative_permeability;
pub mod si;

pub use var_quantity;
pub use var_quantity::uom;
