#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    simplex_2d, simplex_3d, simplex_4d,
    simplex_2d_deriv, simplex_3d_deriv, simplex_4d_deriv,
    Vector2, Vector3, Vector4,
    PermutationTable,
};

criterion_group!(
    simplex,
    bench_simplex2,
    bench_simplex2_deriv,
    bench_simplex3,
    bench_simplex3_deriv,
    bench_simplex4,
    bench_simplex4_deriv,
);
criterion_group!(
    simplex_64x64,
    bench_simplex2_64x64,
    bench_simplex2_deriv_64x64,
    bench_simplex3_64x64,
    bench_simplex3_deriv_64x64,
    bench_simplex4_64x64,
    bench_simplex4_deriv_64x64,
);
criterion_main!(simplex, simplex_64x64);

fn bench_simplex2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 2d", |b| {
        b.iter(|| simplex_2d(black_box(Vector2::new(42.0_f64, 37.0)), &hasher))
    });
}

fn bench_simplex2_deriv(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 2d deriv", |b| {
        b.iter(|| simplex_2d_deriv(black_box(Vector2::new(42.0_f64, 37.0)), &hasher))
    });
}

fn bench_simplex3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 3d", |b| {
        b.iter(|| simplex_3d(black_box(Vector3::new(42.0_f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_simplex3_deriv(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 3d deriv", |b| {
        b.iter(|| simplex_3d_deriv(black_box(Vector3::new(42.0_f64, 37.0, 26.0)), &hasher))
    });
}

fn bench_simplex4(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 4d", |b| {
        b.iter(|| simplex_4d(black_box(Vector4::new(42.0_f64, 37.0, 26.0, 128.0)), &hasher))
    });
}

fn bench_simplex4_deriv(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 4d deriv", |b| {
        b.iter(|| simplex_4d_deriv(black_box(Vector4::new(42.0_f64, 37.0, 26.0, 128.0)), &hasher))
    });
}

fn bench_simplex2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_2d(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_simplex2_deriv_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 2d deriv (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_2d_deriv(Vector2::new(x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_simplex3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_3d(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}

fn bench_simplex3_deriv_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 3d deriv (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_3d_deriv(Vector3::new(x as f64, y as f64, x as f64), &hasher));
                }
            }
        })
    });
}

fn bench_simplex4_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_4d(Vector4::new(x as f64, y as f64, x as f64, y as f64), &hasher));
                }
            }
        })
    });
}

fn bench_simplex4_deriv_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("simplex 4d deriv (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    black_box(simplex_4d_deriv(Vector4::new(x as f64, y as f64, x as f64, y as f64), &hasher));
                }
            }
        })
    });
}
