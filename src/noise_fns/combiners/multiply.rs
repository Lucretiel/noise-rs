use noise_fns::NoiseFn;

/// Noise function that outputs the product of the two output values from two source
/// functions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Multiply<A, B>(pub A, pub B);

impl<A, B> Multiply<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Multiply(lhs, rhs)
    }
}

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Multiply<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point) * self.1.get(point)
    }
}
