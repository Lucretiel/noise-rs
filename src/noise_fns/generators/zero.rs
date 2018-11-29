use noise_fns::NoiseFn;

/// Noise function that always outputs 0
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions. It has the benefit of being 0 size,
/// so it's highly efficient to embed into other Noise combinators.
#[derive(Clone, Copy, Debug, Default)]
pub struct Zero;

impl Zero {
    pub fn new() -> Self {
        Zero
    }
}

impl<T> NoiseFn<T> for Zero {
    #[inline]
    fn get(&self, _point: T) -> f64 {
        0.0
    }
}
