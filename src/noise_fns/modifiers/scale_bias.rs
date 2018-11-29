use noise_fns::NoiseFn;

/// Noise function that applies a scaling factor and a bias to the output value
/// from the source function.
///
/// The function retrieves the output value from the source function, multiplies
/// it with the scaling factor, adds the bias to it, then outputs the value.
#[derive(Debug, Clone, PartialEq)]
pub struct ScaleBias<Source> {
    /// Outputs a value.
    pub source: Source,

    /// Scaling factor to apply to the output value from the source function.
    /// The default value is 1.0.
    pub scale: f64,

    /// Bias to apply to the scaled output value from the source function.
    /// The default value is 0.0.
    pub bias: f64,
}

impl<Source> ScaleBias<Source> {
    pub fn new(source: Source) -> Self {
        ScaleBias {
            source,
            scale: 1.0,
            bias: 0.0,
        }
    }

    pub fn set_scale(self, scale: f64) -> Self {
        ScaleBias { scale, ..self }
    }

    pub fn set_bias(self, bias: f64) -> Self {
        ScaleBias { bias, ..self }
    }
}

impl<Source: Default> Default for ScaleBias<Source> {
    fn default() -> Self {
        Self::new(Source::default())
    }
}

impl<T, Source: NoiseFn<T>> NoiseFn<T> for ScaleBias<Source> {
    #[cfg(not(target_os = "emscripten"))]
    fn get(&self, point: T) -> f64 {
        self.source.get(point).mul_add(self.scale, self.bias)
    }

    #[cfg(target_os = "emscripten")]
    fn get(&self, point: T) -> f64 {
       (self.source.get(point) * self.scale) + self.bias
    }
}
