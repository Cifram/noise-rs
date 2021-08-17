//! An example of using perlin_surflet noise

extern crate noise;

use noise::{
    utils::*,
    core::perlin_surflet::{perlin_surflet_2d, perlin_surflet_3d, perlin_surflet_4d},
    permutationtable::PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(perlin_surflet_2d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin surflet 2d.png");
    PlaneMapBuilder::new_fn(perlin_surflet_3d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin surflet 3d.png");
    PlaneMapBuilder::new_fn(perlin_surflet_4d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin surflet 4d.png");
}
