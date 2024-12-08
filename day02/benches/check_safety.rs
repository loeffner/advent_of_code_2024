use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day02::check_safety_dampened as check_safety;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_check_safety(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);

    let mut arr1: Vec<i32> = (0..100).map(|_| rng.gen_range(0..100)).collect();

    c.bench_function("check_safety (random values)", |b| {
        b.iter(|| check_safety(black_box(&mut arr1)))
    });
}

// Register the benchmark
criterion_group!(benches, benchmark_check_safety);
criterion_main!(benches);
