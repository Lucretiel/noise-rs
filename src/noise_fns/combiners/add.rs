use noise_fns::NoiseFn;

/// Noise function that outputs the sum of the two output values from two source
/// functions.
///
/// # Examples
///
/// ```
/// use noise::{Constant, Add}
///
/// let a = Constant::new(4);
/// let b = Constant::new(5);
/// let noise = Add(a, b);
///
/// assert_eq!(noise([0.0, 0.0, 0.0]), 9.0)
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Add<A, B>(pub A, pub B);

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>> NoiseFn<T> for Add<A, B> {
    fn get(&self, point: T) -> f64 {
        self.0.get(point) + self.1.get(point)
    }
}
