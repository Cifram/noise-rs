use crate::noise_fns::NoiseFn;

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
pub struct ScaleBias<Source, const DIM: usize> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<Source, const DIM: usize> ScaleBias<Source, DIM>
where
    Source: NoiseFn<f64, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn set_scale(self, scale: f64) -> Self {
        Self { scale, ..self }
    }

    pub fn set_bias(self, bias: f64) -> Self {
        Self { bias, ..self }
    }
}

impl<Source, const DIM: usize> NoiseFn<f64, DIM> for ScaleBias<Source, DIM>
where
    Source: NoiseFn<f64, DIM>,
{
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source.get(point)).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: [f64; DIM]) -> f64 {
        (self.source.get(point) * self.scale) + self.bias
    }
}
