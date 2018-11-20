use math::interp;
use noise_fns::NoiseFn;

/// Noise function that outputs a weighted blend of the output values from two
/// source functions given the output value supplied by a control function.
///
/// This noise function uses linear interpolation to perform the blending
/// operation.
pub struct Blend<A, B, C> {
    /// Outputs one of the values to blend.
    pub source1: A,

    /// Outputs one of the values to blend.
    pub source2: B,

    /// Determines the weight of the blending operation. Negative values weight
    /// the blend towards the output value from the `source1` function. Positive
    /// values weight the blend towards the output value from the `source2`
    /// function.
    pub control: C,
}

impl<A, B, C> Blend<A, B, C> {
    pub fn new(source1: A, source2: B, control: C) -> Self {
        Blend {
            source1,
            source2,
            control,
        }
    }
}

impl<T: Copy, A: NoiseFn<T>, B: NoiseFn<T>, C: NoiseFn<T>> NoiseFn<T> for Blend<A, B, C> {
    fn get(&self, point: T) -> f64 {
        let lower = self.source1.get(point);
        let upper = self.source2.get(point);
        let control = self.control.get(point);

        interp::linear(lower, upper, control)
    }
}
