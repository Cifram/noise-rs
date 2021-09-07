use core::fmt;
use rand::{prelude::*, rngs::SmallRng};

const TABLE_SIZE: usize = 256;

/// A seed table, required by all noise functions.
///
/// Table creation is expensive, so in most circumstances you'll only want to
/// create one of these per generator.
#[derive(Copy, Clone)]
pub struct PermutationTable {
    values: [usize; TABLE_SIZE],
}

impl PermutationTable {
    pub fn new(seed: u64) -> Self {
        let mut seq = [0; 256];
        for i in 0..256 {
            seq[i] = i;
        }
        seq.shuffle(&mut SmallRng::seed_from_u64(seed));
        PermutationTable {
            values: seq,
        }
    }

    #[inline(always)]
    pub fn hash_1d(&self, to_hash: isize) -> usize {
        self.values[(to_hash & 0xff) as usize]
    }

    #[inline(always)]
    pub fn hash_2d(&self, to_hash: [isize; 2]) -> usize {
        self.hash_1d(to_hash[1] ^ self.hash_1d(to_hash[0]) as isize)
    }

    #[inline(always)]
    pub fn hash_3d(&self, to_hash: [isize; 3]) -> usize {
        self.hash_1d(to_hash[2] ^ self.hash_2d([to_hash[0], to_hash[1]]) as isize)
    }

    #[inline(always)]
    pub fn hash_4d(&self, to_hash: [isize; 4]) -> usize {
        self.hash_1d(to_hash[3] ^ self.hash_3d([to_hash[0], to_hash[1], to_hash[2]]) as isize)
    }

    #[inline(always)]
    pub fn hash_5d(&self, to_hash: [isize; 5]) -> usize {
        self.hash_1d(to_hash[4] ^ self.hash_4d([to_hash[0], to_hash[1], to_hash[2], to_hash[3]]) as isize)
    }
}

impl fmt::Debug for PermutationTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PermutationTable {{ .. }}")
    }
}
