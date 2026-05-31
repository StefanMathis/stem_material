stem_material
=============

<!-- This file has ben generated with build.rs by concatenating docs/links.md,
docs/main.md and (if available docs/end.md). Do not modify this file, instead
modify the components. -->

[`Material`]: https://docs.rs/stem_material/0.3.4/stem_material/material/struct.Material.html
[`FerromagneticPermeability`]: https://docs.rs/stem_material/0.3.4/stem_material/relative_permeability/struct.FerromagneticPermeability.html
[`JordanModel`]: https://docs.rs/stem_material/0.3.4/stem_material/iron_losses/jordan_model/struct.JordanModel.html
[module]: https://docs.rs/stem_material/0.3.4/stem_material/iron_losses/jordan_model/
[var_quantity]: https://crates.io/crates/var_quantity
[relative_permeability.svg]: https://raw.githubusercontent.com/StefanMathis/stem_material/refs/heads/main/docs/img/relative_permeability.svg
[jordan_model.svg]: https://raw.githubusercontent.com/StefanMathis/stem_material/refs/heads/main/docs/img/jordan_model.svg

[![Documentation](https://docs.rs/stem_material/badge.svg)](https://docs.rs/stem_material)

Material definition for stem - a Simulation Toolbox for Electric Motors.

The full API documentation is available at <https://docs.rs/stem_material/0.3.4/stem_material>.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on [GitHub](https://github.com/StefanMathis/stem_material.git).

This crate provides a [`Material`] definition for simulation of electromagnetic
devices – especially electric motors – built on top of the [var_quantity] crate.
It is used to define motor components within stem - a Simulation Toolbox for
Electric Motors. See the [stem book](https://stefanmathis.github.io/stem_book/)
for an introduction to the framework.

# Modeling soft magnetism

The following models for soft magnetism can both be used stand-alone and as part
of a [`Material`].

## Ferromagnetic permeability

The [`FerromagneticPermeability`] struct offers a spline-based way to model the
ferromagnetic behaviour of a material from measured datapoints. It is
particularily optimized for usage with iterative solvers and slightly modifies
the resulting curve to achieve numerical stability and fast convergence.
Additionally, it can also take the iron fill factor of lamination sheets into
account.

The struct models both `µr = f(H)` and `µr = f(B)`, meaning that either the
magnetic field strength `H` or the magnetic flux density `B` can be used to
calculate the relative permeability.

The following image shows the (modified) spline derived from raw data both for
an iron fill factor of 100 % and of 95 % (the other 5 % are modeled as air with
a relative permeability of 1).

![Relative permeability][relative_permeability.svg]

## Jordan model for iron losses

The [`JordanModel`] type provides a simple and fast model for calculating
hysteresis and eddy current losses based on the equation
`p = kh * f * B² + kec * (f * B)²`. The accompanying [module]
offers ergonomic ways to obtain the the loss coefficients `kh` and `kec` using
least-square fitting.

Due to the model only having two parameters, its modeling accuracy is limited.
The following image shows the raw loss data for different frequencies and the
interpolated curves created by the according [`JordanModel`]. It can be clearly
seen that the model precision is very good for small frequencies, but degrades
for higher frequencies.

![Jordan model][jordan_model.svg]

# Serialization and deserialization

The serde integration is gated behind the `serde` feature flag.

Most of the types (except errors) in this crate implement serialization and
deserialization. See the docstrings of the individual types for details.

# Documentation

The doc images are created by a second crate located within its repository 
(`docs/create_doc_images`) which uses this crate and the awesome
[plotters](https://crates.io/crates/plotters) crate.
The images shown in this documentation can be created with `cargo run` from
within `docs/create_doc_images`.