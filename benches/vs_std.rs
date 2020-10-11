use alt_dedup::NewDedup;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use rand::{distributions::Standard, prelude::SliceRandom, Rng, SeedableRng};
fn compare<T: Clone + core::cmp::PartialEq>(c: &mut Criterion, name: &str, data: Vec<T>) {
    let mut group = c.benchmark_group(name);
    group.bench_function("std", |b| {
        b.iter_batched(
            || data.clone(),
            |mut data| {
                data.dedup();
                data
            },
            BatchSize::SmallInput,
        );
    });
    group.bench_function("alt", |b| {
        b.iter_batched(
            || data.clone(),
            |mut data| {
                data.new_dedup();
                data
            },
            BatchSize::SmallInput,
        );
    });
    group.finish();
}
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::rngs::SmallRng::seed_from_u64(9438591340573909621);
    compare(
        c,
        "Random bytes",
        (&mut rng)
            .sample_iter(Standard)
            .take(40_000)
            .collect::<Vec<u8>>(),
    );
    let strings = ["Hello", "wonder", "whistle", "Crunch", "sizzle"];
    compare(
        c,
        "Borrowed strings",
        (0..8_000).flat_map(|_| strings.choose(&mut rng)).collect(),
    );
    let strings = [
        "abcdefghijk",
        "abcde0ghijk",
        "ab[defghijk",
        "abcdefg!ijk",
        ">bcdefghijk",
    ];
    compare(
        c,
        "Borrowed strings of the same size",
        (0..8_000).flat_map(|_| strings.choose(&mut rng)).collect(),
    );
    compare(
        c,
        "Owned strings",
        (0..1_000)
            .flat_map(|_| strings.choose(&mut rng).map(|s| s.to_string()))
            .collect(),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
