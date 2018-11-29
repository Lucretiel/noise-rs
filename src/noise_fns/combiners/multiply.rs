use noise_fns::NoiseFn;

/// Noise function that outputs the product of the two output values from two source
/// functions.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Multiply<A, B>(pub A, pub B);

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Multiply<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point) * self.1.get(point)
    }
}
