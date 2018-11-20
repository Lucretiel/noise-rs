use noise_fns::NoiseFn;

/// Noise function that outputs the absolute value of the output value from the
/// source function.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct Abs<A>(pub A);

impl<A> Abs<A> {
    pub fn new(source: A) -> Self {
        Abs(source)
    }
}

impl<T, A: NoiseFn<T>> NoiseFn<T> for Abs<A> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point).abs()
    }
}
