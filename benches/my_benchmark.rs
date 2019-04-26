#[macro_use]
extern crate criterion;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use criterion::Criterion;
use rand::distributions::Uniform;
use rand::prelude::*;
use rand::rngs::SmallRng;
use sorted_vec::SortedVec;
use std::collections::BTreeSet;

const SEED_STR: &'static str = "cafebabe";
const SEED_SIZE: usize = 16;
lazy_static! {
    static ref SEED: [u8; SEED_SIZE] = {
        let mut s: [u8; SEED_SIZE] = Default::default();
        s[0..SEED_STR.bytes().len()].copy_from_slice(SEED_STR.as_bytes());
        s
    };
    static ref SIZES: Vec<usize> = (10..=26).map(|i| 1 << i).collect::<Vec<_>>();
}

fn my_bench(c: &mut Criterion) {
    c.bench_function_over_inputs(
        "Vec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::from_seed(*SEED);
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
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "BTreeSet",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::from_seed(*SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: BTreeSet<_> = iter.take(n as usize).collect();
            b.iter(|| {
                let v = rng.next_u64() as usize;
                s.insert(v);
                s.remove(&v);
            });
        },
        SIZES.clone(),
    )
    .bench_function_over_inputs(
        "SortedVec",
        |b, &n| {
            let mut rng: SmallRng = SeedableRng::from_seed(*SEED);
            let range = Uniform::new_inclusive(1, n * n);
            let iter = rng.sample_iter(&range);
            let mut s: SortedVec<_> = iter.take(n as usize).collect();
            b.iter(|| {
                let v = rng.next_u64() as usize;
                s.insert(v);
                s.remove(&v);
            });
        },
        SIZES.clone(),
    );
}

criterion_group!(benches, my_bench);
criterion_main!(benches);
