#[derive(Debug)]
pub struct ProbabilityVector<T, const N: usize> {
    _states: [T; N],
    _probabilities: [f64; N],
}

impl<T, const N: usize> ProbabilityVector<T, N> {
    /// Initializes a new ProbabilityVector from a tuple.
    ///
    /// # Safety
    /// Caller guarantees all probabilities sum to 1.
    pub unsafe fn new_unchecked(src: [(T, f64); N]) -> Self {
        use std::mem::{self, MaybeUninit};

        let (keys, values) = {
            let mut keys: [MaybeUninit<T>; N] = MaybeUninit::uninit().assume_init();
            let mut values: [f64; N] = [0.0; N];

            let iter = keys[..]
                .iter_mut()
                .zip(values.iter_mut())
                .zip(src.into_iter());

            for ((dest_key, dest_val), (src_key, src_val)) in iter {
                dest_key.write(src_key);
                *dest_val = src_val;
            }

            let initialized_keys = mem::transmute_copy::<_, [T; N]>(&keys);
            (initialized_keys, values)
        };

        Self {
            _states: keys,
            _probabilities: values,
        }
    }

    pub fn try_new(src: [(T, f64); N]) -> Option<Self> {
        let probability_sum = src
            .iter()
            .map(|(_, probability)| probability)
            .copied()
            .sum::<f64>();

        if probability_sum == 1_f64 {
            // Satisfies callee guarantee that probability sums to 1.
            let pv = unsafe { Self::new_unchecked(src) };
            Some(pv)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_probability_vector_from_tuple() {
        let pv = ProbabilityVector::try_new([
            ("north", 0.1),
            ("south", 0.4),
            ("east", 0.3),
            ("west", 0.2),
        ]);

        assert!(pv.is_some())
    }
}
