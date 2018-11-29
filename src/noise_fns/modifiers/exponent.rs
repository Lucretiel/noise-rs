use noise_fns::NoiseFn;

use math::scale_shift;

/// Noise function that maps the output value from the source function onto an
/// exponential curve.
///
/// Because most noise functions will output values that range from -1.0 to 1.0,
/// this noise function first normalizes the output value (the range becomes 0.0
/// to 1.0), maps that value onto an exponential curve, then rescales that
/// value back to the original range.
#[derive(Debug, Clone, PartialEq)]
pub struct Exponent<T> {
    /// Outputs a value.
    pub source: T,

    /// Exponent to apply to the output value from the source function. Default
    /// is 1.0.
    pub exponent: f64,
}

impl<T> Exponent<T> {
    pub fn new(source: T) -> Self {
        Exponent {
            source,
            exponent: 1.0,
        }
    }

    pub fn set_exponent(self, exponent: f64) -> Self {
        Exponent { exponent, ..self }
    }
}

impl<T: Default> Default for Exponent<T> {
    fn default() -> Self {
        Exponent::new(T::default())
    }
}

impl<T, S: NoiseFn<T>> NoiseFn<T> for Exponent<S> {
    fn get(&self, point: T) -> f64 {
        let mut value = self.source.get(point);
        value = (value + 1.0) / 2.0;
        value = value.abs();
        value = value.powf(self.exponent);
        scale_shift(value, 2.0)
    }
}
