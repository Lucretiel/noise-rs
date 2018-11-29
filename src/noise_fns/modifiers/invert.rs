use noise_fns::NoiseFn;

/// Noise function that inverts the output value from the source function.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Invert<Source>(pub Source);

impl<T, Source: NoiseFn<T>> NoiseFn<T> for Invert<Source> {
    fn get(&self, point: T) -> f64 {
        1.0 / self.0.get(point)
    }
}
