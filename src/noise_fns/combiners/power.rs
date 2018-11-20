use noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Power<A, B>(pub A, pub B);

impl<A, B> Power<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Power(lhs, rhs)
    }
}

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Power<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point).powf(self.1.get(point))
    }
}
