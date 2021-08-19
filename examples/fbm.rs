//! An example of using perlin noise

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

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_2d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_3d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_4d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin 4d.png");

    PlaneMapBuilder::new_fn(|point, hasher| fbm_open_simplex_2d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_open_simplex_3d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_open_simplex_4d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm open simplex 4d.png");

    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_surflet_2d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin surflet 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_surflet_3d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin surflet 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_perlin_surflet_4d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm perlin surflet 4d.png");

    PlaneMapBuilder::new_fn(|point, hasher| fbm_simplex_2d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm simplex 2d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_simplex_3d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm simplex 3d.png");
    PlaneMapBuilder::new_fn(|point, hasher| fbm_simplex_4d(point, 1.0, 2.0, 0.5, 6, hasher), &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("fbm simplex 4d.png");
}
