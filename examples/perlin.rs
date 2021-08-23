//! An example of using perlin noise

extern crate noise;

use noise::{utils::*, core::perlin::{perlin_2d, perlin_3d, perlin_4d}, permutationtable::PermutationTable};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point| perlin_2d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin 2d.png");
    PlaneMapBuilder::new_fn(|point| perlin_3d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin 3d.png");
    PlaneMapBuilder::new_fn(|point| perlin_4d(point.into(), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("perlin 4d.png");
}
