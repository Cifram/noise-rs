#[macro_use]
extern crate criterion;
extern crate noise;

use criterion::{black_box, Criterion};
use noise::{
    core::perlin_surflet::{perlin_surflet_2d, perlin_surflet_3d, perlin_surflet_4d},
    permutationtable::PermutationTable,
};

criterion_group!(
    perlin_surflet,
    bench_perlin_surflet2,
    bench_perlin_surflet3,
    bench_perlin_surflet4
);
criterion_group!(
    perlin_surflet_64x64,
    bench_perlin_surflet2_64x64,
    bench_perlin_surflet3_64x64,
    bench_perlin_surflet4_64x64
);
criterion_main!(perlin_surflet, perlin_surflet_64x64);

fn bench_perlin_surflet2(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 2d", |b| {
        b.iter(|| perlin_surflet_2d(black_box([42.0_f64, 37.0]), &hasher))
    });
}

fn bench_perlin_surflet3(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 3d", |b| {
        b.iter(|| perlin_surflet_3d(black_box([42.0_f64, 37.0, 26.0]), &hasher))
    });
}

fn bench_perlin_surflet4(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 4d", |b| {
        b.iter(|| perlin_surflet_4d(black_box([42.0_f64, 37.0, 26.0, 128.0]), &hasher))
    });
}

fn bench_perlin_surflet2_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 2d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_surflet_2d(black_box([x as f64, y as f64]), &hasher);
                }
            }
        })
    });
}

fn bench_perlin_surflet3_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 3d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_surflet_3d(black_box([x as f64, y as f64, x as f64]), &hasher);
                }
            }
        })
    });
}

fn bench_perlin_surflet4_64x64(c: &mut Criterion) {
    let hasher = PermutationTable::new(0);
    c.bench_function("perlin surflet 4d (64x64)", |b| {
        b.iter(|| {
            for y in 0i8..64 {
                for x in 0i8..64 {
                    perlin_surflet_4d(black_box([x as f64, y as f64, x as f64, y as f64]), &hasher);
                }
            }
        })
    });
}
