use noise_fns::NoiseFn;

/// Noise function that outputs the smaller of the two output values from two source
/// functions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Min<A, B>(pub A, pub B);

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Min<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point).min(self.1.get(point))
    }
}
