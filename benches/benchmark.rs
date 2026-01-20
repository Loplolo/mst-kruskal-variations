use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use mst_kruskal_variants::{
    FilterKruskal, GraphMatrix, GraphStars, Kruskal, QuickSortKruskal, SkewedFilterKruskal,
    StarQuickSortKruskal,
};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn kruskal_comparison_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("kruskal-variants");

    group.sample_size(10);

    let graph_sizes = [
        (100, 500),
        (500, 2_000),
        (1_000, 5_000),
        (2_000, 10_000),
        (5_000, 25_000),
    ];

    let weight_min = 1;
    let weight_max = 1000;
    const SEED: u64 = 0;

    for &(v, e) in &graph_sizes {
        let max_possible_edges = v * (v - 1) / 2;
        let target_edges = std::cmp::min(e, max_possible_edges);

        let p = if max_possible_edges > 0 {
            target_edges as f64 / max_possible_edges as f64
        } else {
            0.0
        };

        // Same seed for both graph generators to ensure the graphs
        // represent the same topology.
        let mut rng_stars = StdRng::seed_from_u64(42);
        let mut rng_matrix = StdRng::seed_from_u64(42);

        let graph_stars =
            GraphStars::<usize>::new_random(0..v, p, weight_min, weight_max, true, &mut rng_stars)
                .unwrap();
        let graph_matrix = GraphMatrix::<usize>::new_random(
            0..v,
            p,
            weight_min,
            weight_max,
            true,
            &mut rng_matrix,
        )
        .unwrap();

        let input_str = format!("{}-v-{}-e", v, e);

        group.bench_with_input(
            BenchmarkId::new("StandardHeap", &input_str),
            &graph_matrix,
            |b, g| {
                b.iter_batched(
                    || Kruskal::new(g),
                    |mut algo| black_box(algo.run()),
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("QuickSort", &input_str),
            &graph_matrix,
            |b, g| {
                b.iter_batched(
                    || (QuickSortKruskal::new(g), StdRng::seed_from_u64(SEED)),
                    |(mut algo, mut rng)| black_box(algo.run(&mut rng)),
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("Filter", &input_str),
            &graph_matrix,
            |b, g| {
                b.iter_batched(
                    || (FilterKruskal::new(g), StdRng::seed_from_u64(SEED)),
                    |(mut algo, mut rng)| black_box(algo.run(&mut rng)),
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("SkewedFilter", &input_str),
            &graph_matrix,
            |b, g| {
                b.iter_batched(
                    || (SkewedFilterKruskal::new(g), StdRng::seed_from_u64(SEED)),
                    |(mut algo, mut rng)| black_box(algo.run(&mut rng)),
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_with_input(
            BenchmarkId::new("StarQS", &input_str),
            &graph_stars,
            |b, g| {
                b.iter_batched(
                    || StarQuickSortKruskal::new(g),
                    |mut algo| black_box(algo.run()),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, kruskal_comparison_benchmark);
criterion_main!(benches);
