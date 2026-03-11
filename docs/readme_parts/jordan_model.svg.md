## Jordan model for iron losse

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