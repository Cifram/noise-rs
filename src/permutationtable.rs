use alloc::vec::Vec;
use core::fmt;
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng, SeedableRng,
};
use rand_xorshift::XorShiftRng;

const TABLE_SIZE: usize = 256;

pub trait NoiseHasher: Send + Sync {
    fn hash_1d(&self, to_hash: isize) -> usize;
    fn hash_2d(&self, to_hash: [isize; 2]) -> usize;
    fn hash_3d(&self, to_hash: [isize; 3]) -> usize;
    fn hash_4d(&self, to_hash: [isize; 4]) -> usize;
    fn hash_5d(&self, to_hash: [isize; 5]) -> usize;
}

/// A seed table, required by all noise functions.
///
/// Table creation is expensive, so in most circumstances you'll only want to
/// create one of these per generator.
#[derive(Copy, Clone)]
pub struct PermutationTable {
    values: [usize; TABLE_SIZE],
}

impl Distribution<PermutationTable> for Standard {
    /// Generates a PermutationTable using a random seed.
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PermutationTable {
        let mut seq: Vec<usize> = (0..TABLE_SIZE).collect();
        seq.shuffle(rng);

        // It's unfortunate that this double-initializes the array, but Rust
        // doesn't currently provide a clean way to do this in one pass. Hopefully
        // it won't matter, as Seed creation will usually be a one-time event.
        let mut perm_table = PermutationTable {
            values: [0; TABLE_SIZE],
        };
        let seq_it = seq.iter();
        for (x, y) in perm_table.values.iter_mut().zip(seq_it) {
            *x = *y
        }
        perm_table
    }
}

impl PermutationTable {
    /// Deterministically generates a new permutation table based on a `u32` seed value.
    ///
    /// Internally this uses a `XorShiftRng`, but we don't really need to worry
    /// about cryptographic security when working with procedural noise.
    pub fn new(seed: u32) -> Self {
        let mut real = [0; 16];
        real[0] = 1;
        for i in 1..4 {
            real[i * 4] = seed as u8;
            real[(i * 4) + 1] = (seed >> 8) as u8;
            real[(i * 4) + 2] = (seed >> 16) as u8;
            real[(i * 4) + 3] = (seed >> 24) as u8;
        }
        let mut rng: XorShiftRng = SeedableRng::from_seed(real);
        rng.gen()
    }
}

impl NoiseHasher for PermutationTable {
    #[inline(always)]
    fn hash_1d(&self, to_hash: isize) -> usize {
        self.values[(to_hash & 0xff) as usize]
    }

    #[inline(always)]
    fn hash_2d(&self, to_hash: [isize; 2]) -> usize {
        self.hash_1d(to_hash[1] ^ self.hash_1d(to_hash[0]) as isize)
    }

    #[inline(always)]
    fn hash_3d(&self, to_hash: [isize; 3]) -> usize {
        self.hash_1d(to_hash[2] ^ self.hash_2d([to_hash[0], to_hash[1]]) as isize)
    }

    #[inline(always)]
    fn hash_4d(&self, to_hash: [isize; 4]) -> usize {
        self.hash_1d(to_hash[3] ^ self.hash_3d([to_hash[0], to_hash[1], to_hash[2]]) as isize)
    }

    #[inline(always)]
    fn hash_5d(&self, to_hash: [isize; 5]) -> usize {
        self.hash_1d(to_hash[4] ^ self.hash_4d([to_hash[0], to_hash[1], to_hash[2], to_hash[3]]) as isize)
    }
}

impl fmt::Debug for PermutationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PermutationTable {{ .. }}")
    }
}

#[cfg(test)]
mod tests {
    use crate::{NoiseFn, Perlin, Seedable};
    use rand::random;

    #[test]
    fn test_random_seed() {
        let perlin = Perlin::default().set_seed(random());
        let _ = perlin.get([1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_negative_params() {
        let perlin = Perlin::default();
        let _ = perlin.get([-1.0, 2.0, 3.0]);
    }
}
