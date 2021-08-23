use crate::{
    core::perlin::{perlin_2d_variant, perlin_3d_variant, perlin_4d_variant},
    core::perlin_surflet::{perlin_surflet_2d_variant, perlin_surflet_3d_variant, perlin_surflet_4d_variant},
    core::open_simplex::{open_simplex_2d_variant, open_simplex_3d_variant, open_simplex_4d_variant},
    core::simplex::{simplex_2d_variant, simplex_3d_variant, simplex_4d_variant},
    permutationtable::PermutationTable,
};

pub fn fbm_2d<F>(point: [f64; 2], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 2], usize) -> f64
{
    let [mut x, mut y] = point;
    x *= frequency;
    y *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        scale *= persistence;
    }
    result
}

pub fn fbm_3d<F>(point: [f64; 3], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 3], usize) -> f64
{
    let [mut x, mut y, mut z] = point;
    x *= frequency;
    y *= frequency;
    z *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y, z], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        scale *= persistence;
    }
    result
}

pub fn fbm_4d<F>(point: [f64; 4], frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
where
    F: Fn([f64; 4], usize) -> f64
{
    let [mut x, mut y, mut z, mut w] = point;
    x *= frequency;
    y *= frequency;
    z *= frequency;
    w *= frequency;
    let mut scale = 1.0;
    let mut result = 0.0;
    for octave in 0..octaves {
        result += noise_fn([x, y, z, w], octave) * scale;
        x *= lacunarity;
        y *= lacunarity;
        z *= lacunarity;
        w *= lacunarity;
        scale *= persistence;
    }
    result
}

pub fn fbm_perlin_2d(
    point: [f64; 2],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_2d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_2d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_perlin_3d(
    point: [f64; 3],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_3d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_3d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_perlin_4d(
    point: [f64; 4],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_4d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_4d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_open_simplex_2d(
    point: [f64; 2],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_2d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| open_simplex_2d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_open_simplex_3d(
    point: [f64; 3],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_3d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| open_simplex_3d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_open_simplex_4d(
    point: [f64; 4],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_4d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| open_simplex_4d_variant(point.into(), octave as isize, hasher)
    )
}

pub fn fbm_perlin_surflet_2d(
    point: [f64; 2],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_2d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_surflet_2d_variant(point, octave as isize, hasher)
    )
}

pub fn fbm_perlin_surflet_3d(
    point: [f64; 3],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_3d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_surflet_3d_variant(point, octave as isize, hasher)
    )
}

pub fn fbm_perlin_surflet_4d(
    point: [f64; 4],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_4d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| perlin_surflet_4d_variant(point, octave as isize, hasher)
    )
}

pub fn fbm_simplex_2d(
    point: [f64; 2],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_2d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| simplex_2d_variant(point, octave as isize, hasher)
    )
}

pub fn fbm_simplex_3d(
    point: [f64; 3],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_3d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| simplex_3d_variant(point, octave as isize, hasher)
    )
}

pub fn fbm_simplex_4d(
    point: [f64; 4],
    frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
) -> f64 {
    fbm_4d(
        point, frequency, lacunarity, persistence, octaves,
        |point, octave| simplex_4d_variant(point, octave as isize, hasher)
    )
}
