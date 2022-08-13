use criterion::{black_box, criterion_group, criterion_main, Criterion};
use frame_view_tools_lib::*;
use std::path::Path;
use core::time;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialize group");
    
    group.sample_size(10).warm_up_time(time::Duration::from_secs(15));
    group.bench_function("deserialize large csv", |b| b.iter(|| deserialize_csv_into_vec(black_box(Path::new("./benches/bench_files/large.csv")))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);