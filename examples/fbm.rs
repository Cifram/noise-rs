//! An example of using perlin noise

extern crate noise;

use noise::{
    utils::*,
    core::perlin::{perlin_2d, perlin_3d, perlin_4d},
    core::open_simplex::{open_simplex_2d, open_simplex_3d, open_simplex_4d},
    core::fbm::{fbm_2d, fbm_3d, fbm_4d},
    permutationtable::PermutationTable,
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point, hasher| fbm_2d(point, 1.0, 2.0, 0.5, 6, |point, _| perlin_2d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_3d(point, 1.0, 2.0, 0.5, 6, |point, _| perlin_3d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_4d(point, 1.0, 2.0, 0.5, 6, |point, _| perlin_4d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 4d.png");

    PlaneMapBuilder::new_fn(|point, hasher| fbm_2d(point, 1.0, 2.0, 0.5, 6, |point, _| open_simplex_2d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_3d(point, 1.0, 2.0, 0.5, 6, |point, _| open_simplex_3d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_4d(point, 1.0, 2.0, 0.5, 6, |point, _| open_simplex_4d(point, hasher)), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 4d.png");
}
