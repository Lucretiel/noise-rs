use noise_fns::NoiseFn;

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Constant(pub f64);

impl Constant {
    pub fn new(value: impl Into<f64>) -> Self {
        Constant(value.into())
    }
}

impl<T> NoiseFn<T> for Constant {
    #[inline]
    fn get(&self, _point: T) -> f64 {
        self.0
    }
}
