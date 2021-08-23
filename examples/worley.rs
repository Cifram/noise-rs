extern crate noise;

use noise::{
    core::worley::{distance_functions::*, ReturnType, worley_2d, worley_3d, worley_4d},
    math::vectors::{Vector2, Vector3, Vector4},
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

fn output_2d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&Vector2<f64>, &Vector2<f64>) -> f64,
{
    let hasher = PermutationTable::new(0);
    output(|point| worley_2d(&hasher, distance_function, return_type, point.into()), name);
}

fn output_3d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&Vector3<f64>, &Vector3<f64>) -> f64,
{
    let hasher = PermutationTable::new(0);
    output(|point| worley_3d(&hasher, distance_function, return_type, point.into()), name);
}

fn output_4d<F>(distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&Vector4<f64>, &Vector4<f64>) -> f64,
{
    let hasher = PermutationTable::new(0);
    output(|point| worley_4d(&hasher, distance_function, return_type, point.into()), name);
}

fn main() {
    output_2d(&euclidean_2d, ReturnType::Value, "worley 2d euclidean value.png");
    output_3d(&euclidean_3d, ReturnType::Value, "worley 3d euclidean value.png");
    output_4d(&euclidean_4d, ReturnType::Value, "worley 4d euclidean value.png");
    output_2d(&euclidean_2d, ReturnType::Distance, "worley 2d euclidean distance.png");
    output_3d(&euclidean_3d, ReturnType::Distance, "worley 3d euclidean distance.png");
    output_4d(&euclidean_4d, ReturnType::Distance, "worley 4d euclidean distance.png");
    output_2d(&euclidean_squared_2d, ReturnType::Value, "worley 2d euclidean squared value.png");
    output_3d(&euclidean_squared_3d, ReturnType::Value, "worley 3d euclidean squared value.png");
    output_4d(&euclidean_squared_4d, ReturnType::Value, "worley 4d euclidean squared value.png");
    output_2d(&euclidean_squared_2d, ReturnType::Distance, "worley 2d euclidean squared distance.png");
    output_3d(&euclidean_squared_3d, ReturnType::Distance, "worley 3d euclidean squared distance.png");
    output_4d(&euclidean_squared_4d, ReturnType::Distance, "worley 4d euclidean squared distance.png");
}
