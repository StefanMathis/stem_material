This crate provides a [`Material`] definition for simulation of electromagnetic
devices – especially electric motors – built on top of the [var_quantity] crate.
It is used to define motor components within stem - a Simulation Toolbox for
Electric Motors. See the [stem book](https://stefanmathis.github.io/stem_book/)
for an introduction to the framework.

> **Feedback welcome!**  
> Found a bug, missing docs, or have a feature request?  
> Please open an issue on GitHub.

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