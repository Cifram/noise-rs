extern crate noise;

use noise::{
    core::worley::{distance_functions::*, ReturnType, worley_2d, worley_3d, worley_4d},
    permutationtable::PermutationTable,
    utils::*,
};

fn output<F, const DIM: usize>(closure: F, hasher: &PermutationTable, name: &str)
where
    F: Fn([f64; DIM], &PermutationTable) -> f64,
{
    PlaneMapBuilder::new_fn(closure, &hasher)
        .set_size(1024, 1024)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file(name);
}

fn output_2d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 2], hasher: &PermutationTable| worley_2d(hasher, distance_function, return_type, point);
    output(closure, &hasher, name);
}

fn output_3d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 3], hasher: &PermutationTable| worley_3d(hasher, distance_function, return_type, point);
    output(closure, &hasher, name);
}

fn output_4d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&[f64], &[f64]) -> f64,
{
    let hasher = PermutationTable::new(0);
    let closure = |point: [f64; 4], hasher: &PermutationTable| worley_4d(hasher, distance_function, return_type, point);
    output(closure, &hasher, name);
}

fn main() {
    output_2d(&euclidean, ReturnType::Value, "worley 2d euclidean value.png");
    output_3d(&euclidean, ReturnType::Value, "worley 3d euclidean value.png");
    output_4d(&euclidean, ReturnType::Value, "worley 4d euclidean value.png");
    output_2d(&euclidean, ReturnType::Distance, "worley 2d euclidean distance.png");
    output_3d(&euclidean, ReturnType::Distance, "worley 3d euclidean distance.png");
    output_4d(&euclidean, ReturnType::Distance, "worley 4d euclidean distance.png");
    output_2d(&euclidean_squared, ReturnType::Value, "worley 2d euclidean squared value.png");
    output_3d(&euclidean_squared, ReturnType::Value, "worley 3d euclidean squared value.png");
    output_4d(&euclidean_squared, ReturnType::Value, "worley 4d euclidean squared value.png");
    output_2d(&euclidean_squared, ReturnType::Distance, "worley 2d euclidean squared distance.png");
    output_3d(&euclidean_squared, ReturnType::Distance, "worley 3d euclidean squared distance.png");
    output_4d(&euclidean_squared, ReturnType::Distance, "worley 4d euclidean squared distance.png");
    output_2d(&manhattan, ReturnType::Value, "worley 2d manhattan value.png");
    output_3d(&manhattan, ReturnType::Value, "worley 3d manhattan value.png");
    output_4d(&manhattan, ReturnType::Value, "worley 4d manhattan value.png");
    output_2d(&manhattan, ReturnType::Distance, "worley 2d manhattan distance.png");
    output_3d(&manhattan, ReturnType::Distance, "worley 3d manhattan distance.png");
    output_4d(&manhattan, ReturnType::Distance, "worley 4d manhattan distance.png");
    output_2d(&chebyshev, ReturnType::Value, "worley 2d chebyshev value.png");
    output_3d(&chebyshev, ReturnType::Value, "worley 3d chebyshev value.png");
    output_4d(&chebyshev, ReturnType::Value, "worley 4d chebyshev value.png");
    output_2d(&chebyshev, ReturnType::Distance, "worley 2d chebyshev distance.png");
    output_3d(&chebyshev, ReturnType::Distance, "worley 3d chebyshev distance.png");
    output_4d(&chebyshev, ReturnType::Distance, "worley 4d chebyshev distance.png");
    output_2d(&quadratic, ReturnType::Value, "worley 2d quadratic value.png");
    output_3d(&quadratic, ReturnType::Value, "worley 3d quadratic value.png");
    output_4d(&quadratic, ReturnType::Value, "worley 4d quadratic value.png");
    output_2d(&quadratic, ReturnType::Distance, "worley 2d quadratic distance.png");
    output_3d(&quadratic, ReturnType::Distance, "worley 3d quadratic distance.png");
    output_4d(&quadratic, ReturnType::Distance, "worley 4d quadratic distance.png");
}
