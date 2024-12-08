use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day02::check_safety_dampened;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_check_safety_dampened(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);

    let mut arr1: Vec<i32> = (0..100).map(|_| rng.gen_range(0..100)).collect();

    c.bench_function("check_safety_dampened (random values)", |b| {
        b.iter(|| check_safety_dampened(black_box(&mut arr1)))
    });
}

// Register the benchmark
criterion_group!(benches, benchmark_check_safety_dampened);
criterion_main!(benches);
