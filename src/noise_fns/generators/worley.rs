use crate::{
    core::worley::*,
    math::vectors::*,
    noise_fns::{NoiseFn, Seedable},
    permutationtable::PermutationTable,
};

/// Noise function that outputs Worley noise.
#[derive(Clone, Copy)]
pub struct Worley {
    /// Frequency of the seed points.
    pub frequency: f64,
    seed: u32,
    perm_table: PermutationTable,
}

impl Worley {
    pub const DEFAULT_SEED: u32 = 0;
    pub const DEFAULT_FREQUENCY: f64 = 1.0;

    pub fn new(seed: u32) -> Self {
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            frequency: Self::DEFAULT_FREQUENCY,
        }
    }

    /// Sets the frequency of the seed points.
    pub fn set_frequency(self, frequency: f64) -> Self {
        Self { frequency, ..self }
    }
}

impl Default for Worley {
    fn default() -> Self {
        Self::new(0)
    }
}

impl Seedable for Worley {
    /// Sets the seed value used by the Worley cells.
    fn set_seed(self, seed: u32) -> Self {
        // If the new seed is the same as the current seed, just return self.
        if self.seed == seed {
            return self;
        }

        // Otherwise, regenerate the permutation table based on the new seed.
        Self {
            perm_table: PermutationTable::new(seed),
            seed,
            ..self
        }
    }

    fn seed(&self) -> u32 {
        self.seed
    }
}

impl NoiseFn<f64, 2> for Worley {
    fn get(&self, point: [f64; 2]) -> f64 {
        let point = Vector2::from(point) * self.frequency;
        worley_2d_range(point, &self.perm_table)
    }
}

impl NoiseFn<f64, 3> for Worley {
    fn get(&self, point: [f64; 3]) -> f64 {
        let point = Vector3::from(point) * self.frequency;
        worley_3d_range(point, &self.perm_table)
    }
}

#[allow(clippy::cognitive_complexity)]
impl NoiseFn<f64, 4> for Worley {
    fn get(&self, point: [f64; 4]) -> f64 {
        let point = Vector4::from(point) * self.frequency;
        worley_4d_range(point, &self.perm_table)
    }
}
