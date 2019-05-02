#![feature(const_int_conversion)]

#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use criterion::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand::rngs::SmallRng;
use sorted_vec::SortedVec;
use std::collections::BTreeSet;

const SEED: u64 = u64::from_be_bytes(*b"cafebabe");
lazy_static! {
    static ref SIZES: Vec<usize> = (10..=27).map(|i| 1 << i).collect();
}

fn find(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Find_Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            s.sort_unstable();
            let v = rng.next_u64() as usize;
            s.push(v);
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
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
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
        "Find_SortedVec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: SortedVec<_> = iter.take(n as usize).collect();
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

fn insert_remove(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "InsertRemove_Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            s.sort_unstable();
            b.iter(|| {
                let v = rng.next_u64() as usize;
                let pos = s.binary_search(&v).err().unwrap();
                s.insert(pos, v);
                let pos = s.binary_search(&v).ok().unwrap();
                s.remove(pos);
                black_box(&s);
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "InsertRemove_BTreeSet",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: BTreeSet<_> = iter.take(n as usize).collect();
            b.iter(|| {
                let v = rng.next_u64() as usize;
                s.insert(v);
                s.remove(&v);
                black_box(&s);
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "InsertRemove_SortedVec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: SortedVec<_> = iter.take(n as usize).collect();
            b.iter(|| {
                let v = rng.next_u64() as usize;
                s.insert(v);
                s.remove(&v);
                black_box(&s);
            });
        },
        SIZES.clone(),
    );
}

fn insert(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Insert_Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            s.sort_unstable();
            let v = rng.next_u64() as usize;
            b.iter_batched_ref(|| s.clone(), |s| {
                let pos = s.binary_search(&v).err().unwrap();
                black_box(s.insert(pos, v));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Insert_BTreeSet",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let s: BTreeSet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            b.iter_batched_ref(|| s.clone(), |s| {
                black_box(s.insert(v));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Insert_SortedVec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let s: SortedVec<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            b.iter_batched_ref(|| s.clone(), |s| {
                black_box(s.insert(v));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    );
}

fn remove(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Remove_Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.push(v);
            s.sort_unstable();
            b.iter_batched_ref(|| s.clone(), |s| {
                let pos = s.binary_search(&v).ok().unwrap();
                black_box(s.remove(pos));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Remove_BTreeSet",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: BTreeSet<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter_batched_ref(|| s.clone(), |s| {
                black_box(s.remove(&v));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "Remove_SortedVec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: SortedVec<_> = iter.take(n as usize).collect();
            let v = rng.next_u64() as usize;
            s.insert(v);
            b.iter_batched_ref(|| s.clone(), |s| {
                black_box(s.remove(&v));
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    );
}

fn construct(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Construct_Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::seed_from_u64(SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: Vec<_> = iter.take(n as usize).collect();
            s.sort_unstable();
            b.iter_batched_ref(|| s.clone(), |s| {
                black_box(s);
            }, BatchSize::SmallInput);
        },
        SIZES.clone(),
    );
}

criterion_group!(benches, insert_remove, insert, remove);
criterion_main!(benches);
