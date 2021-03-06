#[macro_use]
extern crate lazy_static;
extern crate rand;

use criterion::*;
use rand::distributions::Standard;
use rand::prelude::*;
use rand::rngs::SmallRng;
use rotated_array_set::RotatedArraySet;
use std::collections::BTreeSet;

// only works on nightly, uncomment when from_be_bytes is stabilized as a const fn
// const SEED: u64 = u64::from_be_bytes(*b"cafebabe");
lazy_static! {
    // perturb power-of-2 sizes to avoid allocation/cache aliasing artifacts for arrays
    // static ref SIZES: Vec<usize> = {
    //     let mut sizes = Vec::new();
    //     let base = 10;
    //     let increment = 7;
    //     for i in 0..=17 {
    //         for j in 0..=increment {
    //             let size = (1 << (base + i)) + (j * (1 << increment) * (1 << i)) + 10;
    //             sizes.push(size);
    //         }
    //     }
    //     sizes
    // };
    static ref SIZES: Vec<usize> = (10..=20).map(|i| (1 << i) + 10).collect();
}

fn find(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Find_Vec",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.push(v);
            s.sort_unstable();
            b.iter(|| {
                let pos = s.binary_search(&v).ok().unwrap();
                black_box(pos);
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Find_BTreeSet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: BTreeSet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter(|| {
                let r = s.get(&v).unwrap();
                black_box(r);
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Find_RotatedArraySet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: RotatedArraySet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter(|| {
                let r = s.get(&v).unwrap();
                black_box(r);
            });
        },
        SIZES.clone(),
    );
}

fn insert(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Insert_Vec",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            s.sort_unstable();
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    let v = rng.next_u64() as usize;
                    let pos = s.binary_search(&v).err().unwrap();
                    black_box(s.insert(pos, v));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Insert_BTreeSet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let s: BTreeSet<_> = iter.take(n as usize).collect();
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    let v = rng.next_u64() as usize;
                    black_box(s.insert(v));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Insert_RotatedArraySet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let s: RotatedArraySet<_> = iter.take(n as usize).collect();
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    let v = rng.next_u64() as usize;
                    black_box(s.insert(v));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    );
}

fn remove(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Remove_Vec",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.push(v);
            s.sort_unstable();
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    let pos = s.binary_search(&v).ok().unwrap();
                    black_box(s.remove(pos));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Remove_BTreeSet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: BTreeSet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    black_box(s.remove(&v));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Remove_RotatedArraySet",
        |b, &n| {
            // FIXME: remove when const fns are in stable
            let seed: u64 = u64::from_be_bytes(*b"cafebabe");
            let mut rng: SmallRng = SeedableRng::seed_from_u64(seed);
            let iter = rng.sample_iter(&Standard);
            let mut s: RotatedArraySet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter_batched_ref(
                || s.clone(),
                |s| {
                    black_box(s.remove(&v));
                },
                BatchSize::SmallInput,
            );
        },
        SIZES.clone(),
    );
}

criterion_group!(benches, find, insert, remove);
criterion_main!(benches);
