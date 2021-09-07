//! An example of using simplex noise

extern crate noise;

use noise::{
    utils::*,
    turbulance_perlin_2d, turbulance_perlin_3d, turbulance_perlin_4d,
    perlin_2d, perlin_3d, perlin_4d,
    PermutationTable
};

fn main() {
    let hasher = PermutationTable::new(0);
    PlaneMapBuilder::new_fn(|point| perlin_2d(turbulance_perlin_2d(point.into(), 1.0, &hasher), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 2d.png");
    PlaneMapBuilder::new_fn(|point| perlin_3d(turbulance_perlin_3d(point.into(), 1.0, &hasher), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 3d.png");
    PlaneMapBuilder::new_fn(|point| perlin_4d(turbulance_perlin_4d(point.into(), 1.0, &hasher), &hasher))
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file("turbulance perlin 4d.png");
}
