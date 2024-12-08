use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day01::total_distance;
use rand::{rngs::StdRng, Rng, SeedableRng};

fn benchmark_total_distance(c: &mut Criterion) {
    let mut rng = StdRng::seed_from_u64(42);

    let mut arr1: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();
    let mut arr2: Vec<i32> = (0..1000).map(|_| rng.gen_range(0..10000)).collect();

    c.bench_function("total_distance (1000 random values)", |b| {
        b.iter(|| {
            total_distance(black_box(&mut arr1), black_box(&mut arr2))
        })
    });
}

criterion_group!(benches, benchmark_total_distance);
criterion_main!(benches);
