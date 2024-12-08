use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::similarity_score;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_similarity_score(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);

    let mut arr1: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();
    let mut arr2: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();

    c.bench_function("similarity_score (1000 random values)", |b| {
        b.iter(|| {
            similarity_score(black_box(&mut arr1), black_box(&mut arr2))
        })
    });
}

// Register the benchmark
criterion_group!(benches, benchmark_similarity_score);
criterion_main!(benches);
