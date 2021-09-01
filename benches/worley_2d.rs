#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::worley::{worley_2d_value, worley_2d_range, worley_2d_range_sqr, worley_2d_border},
    math::vectors::Vector2,
    permutationtable::PermutationTable,
};

criterion_group!(bench_worley_2d,
    bench_worley2d_euclidean_value,
    bench_worley2d_euclidean_range,
    bench_worley2d_squared_range,
    bench_worley2d_border,
);
criterion_group!(bench_worley_2d_64x64,
    bench_worley2d_euclidean_value_64x64,
    bench_worley2d_euclidean_range_64x64,
    bench_worley2d_squared_range_64x64,
    bench_worley2d_border_64x64,
);
criterion_main!(bench_worley_2d, bench_worley_2d_64x64);

fn bench_worley2d_euclidean_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean value", |b| {
        b.iter(|| worley_2d_value(black_box(Vector2::new(42.0f64, 37.0)), &hasher))
    });
}

fn bench_worley2d_euclidean_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean distance", |b| {
        b.iter(|| worley_2d_range(black_box(Vector2::new(42.0f64, 37.0)), &hasher))
    });
}

fn bench_worley2d_squared_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d squared distance", |b| {
        b.iter(|| worley_2d_range_sqr(black_box(Vector2::new(42.0f64, 37.0)), &hasher))
    });
}

fn bench_worley2d_border(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d border", |b| {
        b.iter(|| worley_2d_border(black_box(Vector2::new(42.0f64, 37.0)), &hasher))
    });
}

fn bench_worley2d_euclidean_value_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_2d_value(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley2d_euclidean_range_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d euclidean distance (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_2d_range(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley2d_squared_range_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d squared distance (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_2d_range_sqr(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley2d_border_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 2d border (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_2d_border(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}
