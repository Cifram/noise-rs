//! Example of using fbm noise

extern crate noise;

use noise::{
    utils::*,
    core::fbm::{
        fbm_perlin_2d, fbm_perlin_3d, fbm_perlin_4d,
        fbm_perlin_surflet_2d, fbm_perlin_surflet_3d, fbm_perlin_surflet_4d,
        fbm_open_simplex_2d, fbm_open_simplex_3d, fbm_open_simplex_4d,
        fbm_simplex_2d, fbm_simplex_3d, fbm_simplex_4d,
    },
    permutationtable::PermutationTable,
};

fn output<F, const DIM: usize>(func: F, name: &str)
where
    F: Fn([f64; DIM]) -> f64
{
    PlaneMapBuilder::new_fn(func)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file(name);
}

fn main() {
    let hasher = PermutationTable::new(0);

    output(|point| fbm_open_simplex_2d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm open simplex 2d.png");
    output(|point| fbm_open_simplex_3d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm open simplex 3d.png");
    output(|point| fbm_open_simplex_4d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm open simplex 4d.png");

    output(|point| fbm_perlin_2d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin 2d.png");
    output(|point| fbm_perlin_3d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin 3d.png");
    output(|point| fbm_perlin_4d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin 4d.png");

    output(|point| fbm_perlin_surflet_2d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin surflet 2d.png");
    output(|point| fbm_perlin_surflet_3d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin surflet 3d.png");
    output(|point| fbm_perlin_surflet_4d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm perlin surflet 4d.png");

    output(|point| fbm_simplex_2d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm simplex 2d.png");
    output(|point| fbm_simplex_3d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm simplex 3d.png");
    output(|point| fbm_simplex_4d(point.into(), 1.0, 2.0, 0.5, 6, &hasher), "fbm simplex 4d.png");
}
