stem_material
=============

[`Material`]: https://docs.rs/stem_material/0.1.0/stem_material/struct.Material.html
[`FerromagneticPermeability`]: https://docs.rs/stem_material/0.1.0/stem_material/ferromagnetic_permeability/struct.FerromagneticPermeability.html
[`JordanModel`]: https://docs.rs/stem_material/0.1.0/stem_material/jordan_model/struct.JordanModel.html
[jordan_module]: https://docs.rs/stem_material/0.1.0/stem_material/jordan_model
[var_quantity]: https://crates.io/crates/var_quantity

This crate provides a [`Material`] definition for simulation of electromagnetic
devices – especially electric motors – built on topof the [var_quantity] crate.
It is used to define motor components within stem - a Simulation Toolbox for
Electric Motors. See the [stem book](https://stefanmathis.github.io/stem_book/)
for an introduction to the framework.

Due to the aforementioned tight coupling with [var_quantity], this crate
reexports it. It also heavily relies on the [uom](https://crates.io/crates/uom)
crate for providing static quantities. The resulting interface is therefore hard
to misuse, as it doesn't matter whether e.g. the density of a material is given
as kg/m³ or as g/cm³ - uom will take care of the conversions automatically.

# Modeling soft magnetism

Even though this crate is foundational for stem, some of its components might
also be useful outside of it. In particular, it offers the following types to
model soft magnetism:
- The [`FerromagneticPermeability`] struct offers a spline-based way to model the
ferromagnetic behaviour of a material from measured datapoints. It is
particularily optimized for usage with iterative solvers and slightly modifies
the resulting curve to achieve numerical stability and fast convergence. It
models both `µr = f(H)` and `µr = f(B)`, meaning that either the magnetic field
strength `H` or the magnetic flux density `B` can be used to calculate the
relative permeability of a material.
- The [`JordanModel`] type provides a simple and fast model for calculating
hysteresis and eddy current losses based on the equation
`p = kh * f * B² + kec * (f * B)²`. The accompanying [module](jordan_module)
offers ergonomic ways to obtain the the loss coefficients `kh` and `kec` using
least-square fitting.

# Serialization and deserialization

The serde integration is gated behind the `serde` feature flag.

Most of the types (except errors) in this crate implement serialization and
deserialization. See the docstrings of the individual types for details.

# Documentation

The full API documentation is available at
[https://docs.rs/stem_material/0.1.0/stem_material/](https://docs.rs/stem_material/0.1.0/stem_material/).