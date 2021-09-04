use crate::{
    core::perlin::{perlin_2d_variant, perlin_3d_variant, perlin_4d_variant},
    core::perlin_surflet::{perlin_surflet_2d_variant, perlin_surflet_3d_variant, perlin_surflet_4d_variant},
    core::open_simplex::{open_simplex_2d_variant, open_simplex_3d_variant, open_simplex_4d_variant},
    core::simplex::{simplex_2d_variant, simplex_3d_variant, simplex_4d_variant},
    math::vectors::*,
    permutationtable::PermutationTable,
};

macro_rules! fbm(
    ($name:ident, $vector_type:ident) => {
        pub fn $name<F>(point: $vector_type<f64>, frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F) -> f64
        where
            F: Fn($vector_type<f64>, usize) -> f64
        {
            let mut point = point * frequency;
            let mut amplitude = 1.0;
            let mut result = 0.0;
            for octave in 0..octaves {
                result += noise_fn(point, octave) * amplitude;
                point *= lacunarity;
                amplitude *= persistence;
            }
            result
        }
    }
);

fbm!(fbm_2d, Vector2);
fbm!(fbm_3d, Vector3);
fbm!(fbm_4d, Vector4);

macro_rules! fbm_ridged(
    ($name:ident, $vector_type:ident, $fbm_fn:ident) => {
        pub fn $name<F>(
            point: $vector_type<f64>,
            frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, noise_fn: F
        ) -> f64
        where
            F: Fn($vector_type<f64>, usize) -> f64
        {
            let mut point = point * frequency;
            let mut amplitude = 1.0;
            let mut result = 0.0;
            let mut weight = 1.0;
            for octave in 0..octaves {
                let mut signal = 1.0 - noise_fn(point, octave).abs();
                signal *= signal * weight;
                weight = (signal / 2.0).clamp(0.0, 1.0);
                result += signal * amplitude;
                point *= lacunarity;
                amplitude *= persistence;
            }
            result * 2.0 / (2.0 - 0.5f64.powi(octaves as i32 - 1)) - 1.0
        }
    }
);

fbm_ridged!(fbm_ridged_2d, Vector2, fbm_weighted_2d);
fbm_ridged!(fbm_ridged_3d, Vector3, fbm_weighted_3d);
fbm_ridged!(fbm_ridged_4d, Vector4, fbm_weighted_4d);

macro_rules! specialized_fbm(
    ($name:ident, $name_variant:ident, $vector_type:ident, $fbm_fn:ident, $noise_fn:ident) => {
        pub fn $name(
            point: $vector_type<f64>,
            frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
        ) -> f64 {
            $fbm_fn(
                point, frequency, lacunarity, persistence, octaves,
                |point, octave| $noise_fn(point.into(), octave as isize, hasher)
            )
        }
        pub fn $name_variant(
            point: $vector_type<f64>, variant: isize,
            frequency: f64, lacunarity: f64, persistence: f64, octaves: usize, hasher: &PermutationTable
        ) -> f64 {
            $fbm_fn(
                point, frequency, lacunarity, persistence, octaves,
                |point, octave| $noise_fn(point.into(), octave as isize + variant*16, hasher)
            )
        }
    }
);

specialized_fbm!(fbm_perlin_2d, fbm_perlin_2d_variant, Vector2, fbm_2d, perlin_2d_variant);
specialized_fbm!(fbm_perlin_3d, fbm_perlin_3d_variant, Vector3, fbm_3d, perlin_3d_variant);
specialized_fbm!(fbm_perlin_4d, fbm_perlin_4d_variant, Vector4, fbm_4d, perlin_4d_variant);
specialized_fbm!(fbm_open_simplex_2d, fbm_open_simplex_2d_variant, Vector2, fbm_2d, open_simplex_2d_variant);
specialized_fbm!(fbm_open_simplex_3d, fbm_open_simplex_3d_variant, Vector3, fbm_3d, open_simplex_3d_variant);
specialized_fbm!(fbm_open_simplex_4d, fbm_open_simplex_4d_variant, Vector4, fbm_4d, open_simplex_4d_variant);
specialized_fbm!(fbm_perlin_surflet_2d, fbm_perlin_surflet_2d_variant, Vector2, fbm_2d, perlin_surflet_2d_variant);
specialized_fbm!(fbm_perlin_surflet_3d, fbm_perlin_surflet_3d_variant, Vector3, fbm_3d, perlin_surflet_3d_variant);
specialized_fbm!(fbm_perlin_surflet_4d, fbm_perlin_surflet_4d_variant, Vector4, fbm_4d, perlin_surflet_4d_variant);
specialized_fbm!(fbm_simplex_2d, fbm_simplex_2d_variant, Vector2, fbm_2d, simplex_2d_variant);
specialized_fbm!(fbm_simplex_3d, fbm_simplex_3d_variant, Vector3, fbm_3d, simplex_3d_variant);
specialized_fbm!(fbm_simplex_4d, fbm_simplex_4d_variant, Vector4, fbm_4d, simplex_4d_variant);

specialized_fbm!(fbm_ridged_perlin_2d, fbm_ridged_perlin_2d_variant, Vector2, fbm_ridged_2d, perlin_2d_variant);
specialized_fbm!(fbm_ridged_perlin_3d, fbm_ridged_perlin_3d_variant, Vector3, fbm_ridged_3d, perlin_3d_variant);
specialized_fbm!(fbm_ridged_perlin_4d, fbm_ridged_perlin_4d_variant, Vector4, fbm_ridged_4d, perlin_4d_variant);
specialized_fbm!(fbm_ridged_open_simplex_2d, fbm_ridged_open_simplex_2d_variant, Vector2, fbm_ridged_2d, open_simplex_2d_variant);
specialized_fbm!(fbm_ridged_open_simplex_3d, fbm_ridged_open_simplex_3d_variant, Vector3, fbm_ridged_3d, open_simplex_3d_variant);
specialized_fbm!(fbm_ridged_open_simplex_4d, fbm_ridged_open_simplex_4d_variant, Vector4, fbm_ridged_4d, open_simplex_4d_variant);
specialized_fbm!(fbm_ridged_perlin_surflet_2d, fbm_ridged_perlin_surflet_2d_variant, Vector2, fbm_ridged_2d, perlin_surflet_2d_variant);
specialized_fbm!(fbm_ridged_perlin_surflet_3d, fbm_ridged_perlin_surflet_3d_variant, Vector3, fbm_ridged_3d, perlin_surflet_3d_variant);
specialized_fbm!(fbm_ridged_perlin_surflet_4d, fbm_ridged_perlin_surflet_4d_variant, Vector4, fbm_ridged_4d, perlin_surflet_4d_variant);
specialized_fbm!(fbm_ridged_simplex_2d, fbm_ridged_simplex_2d_variant, Vector2, fbm_ridged_2d, simplex_2d_variant);
specialized_fbm!(fbm_ridged_simplex_3d, fbm_ridged_simplex_3d_variant, Vector3, fbm_ridged_3d, simplex_3d_variant);
specialized_fbm!(fbm_ridged_simplex_4d, fbm_ridged_simplex_4d_variant, Vector4, fbm_ridged_4d, simplex_4d_variant);
