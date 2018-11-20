use noise_fns::NoiseFn;

/// Noise function that outputs the sum of the two output values from two source
/// functions.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Add<A, B>(pub A, pub B);

impl<A, B> Add<A, B> {
    pub fn new(lhs: A, rhs: B) -> Self {
        Add(lhs, rhs)
    }
}

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Add<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point) + self.1.get(point)
    }
}
