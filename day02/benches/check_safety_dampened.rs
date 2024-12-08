use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day02::check_safety_dampened;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_check_safety_dampened(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);

    let mut vec_of_data: Vec<Vec<i32>> = (0..1000)
        .map(|_| (0..10).map(|_| rng.gen_range(0..100)).collect())
        .collect();

    c.bench_function("check_safety_dampened (1000x10 random values)", |b| {
        b.iter(|| {
            for i in 0..1000 {
                check_safety_dampened(black_box(&mut vec_of_data[i]));
            }
        })
    });
}

// Register the benchmark
criterion_group!(benches, benchmark_check_safety_dampened);
criterion_main!(benches);
