//! An example of using Super Simplex noise

extern crate noise;

use noise::{
    utils::*,
    core::super_simplex::{super_simplex_2d, super_simplex_3d},
    permutationtable::PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(super_simplex_2d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("super simplex 2d.png");
    PlaneMapBuilder::new_fn(super_simplex_3d, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("super simplex 3d.png");
}
