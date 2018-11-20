use noise_fns::NoiseFn;

/// Noise function that outputs the larger of the two output values from two source
/// functions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Max<A, B>(pub A, pub B);

impl<A, B> Max<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Max(lhs, rhs)
    }
}

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Max<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point).max(self.1.get(point))
    }
}
