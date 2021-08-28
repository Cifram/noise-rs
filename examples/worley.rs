extern crate noise;

use noise::{
    core::worley::*,
    permutationtable::PermutationTable,
    utils::*,
};

fn output<F, const DIM: usize>(closure: F, name: &str)
where
    F: Fn([f64; DIM]) -> f64,
{
    PlaneMapBuilder::new_fn(closure)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file(name);
}

fn main() {
    let hasher = PermutationTable::new(738256);
    output(|point| worley_2d_value(point.into(), &hasher), "worley 2d value.png");
    output(|point| worley_2d_range(point.into(), &hasher), "worley 2d range.png");
    output(|point| worley_2d_range_sqr(point.into(), &hasher), "worley 2d range squared.png");
    output(|point| worley_2d_border(point.into(), &hasher), "worley 2d border.png");
    output(|point| worley_3d_value(point.into(), &hasher), "worley 3d value.png");
    output(|point| worley_3d_range(point.into(), &hasher), "worley 3d range.png");
    output(|point| worley_3d_range_sqr(point.into(), &hasher), "worley 3d range squared.png");
    output(|point| worley_3d_border(point.into(), &hasher), "worley 3d border.png");
    output(|point| worley_4d_value(point.into(), &hasher), "worley 4d value.png");
    output(|point| worley_4d_range(point.into(), &hasher), "worley 4d range.png");
    output(|point| worley_4d_range_sqr(point.into(), &hasher), "worley 4d range squared.png");
    output(|point| worley_4d_border(point.into(), &hasher), "worley 4d border.png");
}
