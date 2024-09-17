use crabee::{load_dict, solve};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let chars = &['o', 'b', 'd', 'c', 'k', 'a', 'r'];
    let dict = load_dict("/usr/share/dict/american-english");
    c.bench_function("solve obdckar", |b| {
        b.iter(|| solve(black_box(chars), 'o', 4, &dict))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
