use math;
use noise_fns::NoiseFn;

/// Noise function that clamps the output value from the source function to a
/// range of values.
#[derive(Debug, Clone, PartialEq)]
pub struct Clamp<A> {
    /// Outputs a value.
    pub source: A,

    /// Lower bound of the clamp. Defaults to -1.0
    pub lower_bound: f64,

    /// Upper bound of the clamp. Defaults to 1.0.
    pub upper_bound: f64,
}

impl<A> Clamp<A> {
    pub fn new(source: A) -> Self {
        Self::new_with_bounds(source, -1.0, 1.0)
    }

    pub fn new_with_bounds(source: A, lower_bound: f64, upper_bound: f64) -> Self {
        Clamp {
            source,
            lower_bound,
            upper_bound,
        }
    }

    pub fn set_lower_bound(self, lower_bound: f64) -> Self {
        Clamp {
            lower_bound,
            ..self
        }
    }

    pub fn set_upper_bound(self, upper_bound: f64) -> Self {
        Clamp {
            upper_bound,
            ..self
        }
    }

    pub fn set_bounds(self, lower_bound: f64, upper_bound: f64) -> Self {
        Clamp {
            lower_bound,
            upper_bound,
            ..self
        }
    }
}

impl<A: Default> Default for Clamp<A> {
    fn default() -> Self {
        Self::new(A::default())
    }
}

impl<T, A: NoiseFn<T>> NoiseFn<T> for Clamp<A> {
    fn get(&self, point: T) -> f64 {
        math::clamp(self.source.get(point), self.lower_bound, self.upper_bound)
    }
}
