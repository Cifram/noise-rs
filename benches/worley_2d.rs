#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::worley::{distance_functions::*, ReturnType, worley_2d},
    math::vectors::Vector2,
    permutationtable::PermutationTable,
};

criterion_group!(bench_worley_2d,
    bench_worley2d_euclidean_value,
    bench_worley2d_euclidean_range,
    bench_worley2d_squared_value,
    bench_worley2d_squared_range,
);
criterion_group!(bench_worley_2d_64x64,
    bench_worley2d_euclidean_value_64x64,
    bench_worley2d_euclidean_range_64x64,
    bench_worley2d_squared_value_64x64,
    bench_worley2d_squared_range_64x64,
);
criterion_main!(bench_worley_2d, bench_worley_2d_64x64);

fn bench_worley2d<F>(c: &mut Criterion, distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&Vector2<f64>, &Vector2<f64>) -> f64,
{
    let hasher = PermutationTable::new(0);
    c.bench_function(format!("worley 2d {}", name).as_str(), |b| {
        b.iter(|| worley_2d(&hasher, distance_function, return_type, black_box(Vector2::new(42.0f64, 37.0))))
    });
}

fn bench_worley2d_64x64<F>(c: &mut Criterion, distance_function: &F, return_type: ReturnType, name: &str)
where
    F: Fn(&Vector2<f64>, &Vector2<f64>) -> f64,
{
    let hasher = PermutationTable::new(0);
    c.bench_function(format!("worley 2d {} (64x64)", name).as_str(), |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_2d(&hasher, distance_function, return_type, Vector2::new(x as f64, y as f64)));
                }
            }
        })
    });
}

fn bench_worley2d_euclidean_value(c: &mut Criterion) {
    bench_worley2d(c, &euclidean_2d, ReturnType::Value, "euclidean value");
}

fn bench_worley2d_euclidean_range(c: &mut Criterion) {
    bench_worley2d(c, &euclidean_2d, ReturnType::Distance, "euclidean distance");
}

fn bench_worley2d_squared_value(c: &mut Criterion) {
    bench_worley2d(c, &euclidean_squared_2d, ReturnType::Value, "squared value");
}

fn bench_worley2d_squared_range(c: &mut Criterion) {
    bench_worley2d(c, &euclidean_squared_2d, ReturnType::Distance, "squared distance");
}

fn bench_worley2d_euclidean_value_64x64(c: &mut Criterion) {
    bench_worley2d_64x64(c, &euclidean_2d, ReturnType::Value, "euclidean value");
}

fn bench_worley2d_euclidean_range_64x64(c: &mut Criterion) {
    bench_worley2d_64x64(c, &euclidean_2d, ReturnType::Distance, "euclidean distance");
}

fn bench_worley2d_squared_value_64x64(c: &mut Criterion) {
    bench_worley2d_64x64(c, &euclidean_squared_2d, ReturnType::Value, "squared value");
}

fn bench_worley2d_squared_range_64x64(c: &mut Criterion) {
    bench_worley2d_64x64(c, &euclidean_squared_2d, ReturnType::Distance, "squared distance");
}
