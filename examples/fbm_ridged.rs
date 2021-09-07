//! Example of using fbm ridged noise

extern crate noise;

use noise::{
    utils::*,
    core::fbm::{
        fbm_ridged_perlin_2d, fbm_ridged_perlin_3d, fbm_ridged_perlin_4d,
        fbm_ridged_perlin_surflet_2d, fbm_ridged_perlin_surflet_3d, fbm_ridged_perlin_surflet_4d,
        fbm_ridged_open_simplex_2d, fbm_ridged_open_simplex_3d, fbm_ridged_open_simplex_4d,
        fbm_ridged_simplex_2d, fbm_ridged_simplex_3d, fbm_ridged_simplex_4d,
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

    output(|point| fbm_ridged_open_simplex_2d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged open simplex 2d.png");
    output(|point| fbm_ridged_open_simplex_3d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged open simplex 3d.png");
    output(|point| fbm_ridged_open_simplex_4d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged open simplex 4d.png");

    output(|point| fbm_ridged_perlin_2d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin 2d.png");
    output(|point| fbm_ridged_perlin_3d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin 3d.png");
    output(|point| fbm_ridged_perlin_4d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin 4d.png");

    output(|point| fbm_ridged_perlin_surflet_2d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin surflet 2d.png");
    output(|point| fbm_ridged_perlin_surflet_3d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin surflet 3d.png");
    output(|point| fbm_ridged_perlin_surflet_4d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged perlin surflet 4d.png");

    output(|point| fbm_ridged_simplex_2d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged simplex 2d.png");
    output(|point| fbm_ridged_simplex_3d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged simplex 3d.png");
    output(|point| fbm_ridged_simplex_4d(point.into(), 1.0, 2.0, 1.0, 6, &hasher), "fbm ridged simplex 4d.png");
}
