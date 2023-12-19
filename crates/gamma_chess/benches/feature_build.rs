use criterion::{criterion_group, criterion_main, Criterion};
use gamma_chess::archive::model_0::Dataset as Dataset0;
use gamma_chess::archive::model_1::Dataset as Dataset1;
use gamma_chess::model::Dataset;

fn combined_dataset_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Combined Dataset Pipeline");
    group.sample_size(10);
    let test_file_path = "../../dataset/chunks/0.pb";

    // Benchmark for Dataset
    group.bench_function("Dataset (Main)", |b| {
        b.iter(|| Dataset::new(test_file_path).unwrap())
    });

    // // Benchmark for Dataset2
    // group.bench_function("Dataset2 (Model 0)", |b| {
    //     b.iter(|| Dataset0::new(test_file_path).unwrap())
    // });

    // // Benchmark for Dataset3
    // group.bench_function("Dataset3 (Model 1)", |b| {
    //     b.iter(|| Dataset1::new(test_file_path).unwrap())
    // });

    group.finish();
}

criterion_group!(benches, combined_dataset_benchmark);
criterion_main!(benches);
