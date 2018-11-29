use noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Abs<Source>(pub Source);

impl<T, Source: NoiseFn<T>> NoiseFn<T> for Abs<Source> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point).abs()
    }
}
