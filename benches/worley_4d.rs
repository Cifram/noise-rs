#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::worley::{worley_4d_value, worley_4d_range, worley_4d_range_sqr},
    math::vectors::Vector4,
    permutationtable::PermutationTable,
};

criterion_group!(bench_worley_4d,
    bench_worley4d_euclidean_value,
    bench_worley4d_euclidean_range,
    bench_worley4d_squared_range,
);
criterion_group!(bench_worley_4d_64x64,
    bench_worley4d_euclidean_value_64x64,
    bench_worley4d_euclidean_range_64x64,
    bench_worley4d_squared_range_64x64,
);
criterion_main!(bench_worley_4d, bench_worley_4d_64x64);

fn bench_worley4d_euclidean_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d euclidean value", |b| {
        b.iter(|| worley_4d_value(black_box(Vector4::new(42.0f64, 37.0, 26.0, 128.0)), &hasher))
    });
}

fn bench_worley4d_euclidean_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d euclidean distance", |b| {
        b.iter(|| worley_4d_range(black_box(Vector4::new(42.0f64, 37.0, 26.0, 128.0)), &hasher))
    });
}

fn bench_worley4d_squared_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d squared distance", |b| {
        b.iter(|| worley_4d_range_sqr(black_box(Vector4::new(42.0f64, 37.0, 26.0, 128.0)), &hasher))
    });
}

fn bench_worley4d_euclidean_value_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d euclidean value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_4d_value(Vector4::new(x as f64, y as f64, x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley4d_euclidean_range_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d euclidean distance (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_4d_range(Vector4::new(x as f64, y as f64, x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley4d_squared_range_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 4d squared distance (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_4d_range_sqr(Vector4::new(x as f64, y as f64, x as f64, y as f64), &hasher));
                }
            }
        })
    });
}
