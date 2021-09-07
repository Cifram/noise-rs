#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    worley_3d_value, worley_3d_range, worley_3d_range_sqr, worley_3d_border,
    Vector3,
    PermutationTable,
};

criterion_group!(bench_worley_3d,
    bench_worley3d_value,
    bench_worley3d_range,
    bench_worley3d_range_sqr,
    bench_worley3d_border,
);
criterion_group!(bench_worley_3d_64x64,
    bench_worley3d_value_64x64,
    bench_worley3d_range_64x64,
    bench_worley3d_range_sqr_64x64,
    bench_worley3d_border_64x64,
);
criterion_main!(bench_worley_3d, bench_worley_3d_64x64);

fn bench_worley3d_value(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d value", |b| {
        b.iter(|| worley_3d_value(black_box(Vector3::new(42.0f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_worley3d_range(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d range", |b| {
        b.iter(|| worley_3d_range(black_box(Vector3::new(42.0f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_worley3d_range_sqr(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d range sqr", |b| {
        b.iter(|| worley_3d_range_sqr(black_box(Vector3::new(42.0f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_worley3d_border(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d border", |b| {
        b.iter(|| worley_3d_border(black_box(Vector3::new(42.0f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_worley3d_value_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d value (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_3d_value(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley3d_range_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d range (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_3d_range(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley3d_range_sqr_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d range sqr (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_3d_range_sqr(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}

fn bench_worley3d_border_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("worley 3d border (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(worley_3d_border(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}
