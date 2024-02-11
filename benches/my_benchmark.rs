use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lines::{lines, lines_simd};

pub fn criterion_benchmark(c: &mut Criterion) {
    let string = std::fs::read_to_string("data/9_Swamp_S2B_ITS2_2019_minq7.fastq").unwrap();

    let mut v = vec![];
    c.bench_function("lines", |b| {
        b.iter(|| {
            v.clear();
            let lines = string.lines();
            v.extend(lines);
        })
    });
    let mut v = vec![];
    c.bench_function("lines simd", |b| {
        b.iter(|| {
            v.clear();
            let lines = lines_simd(&string);
            v.extend(lines);
        })
    });
    let mut v = vec![];
    c.bench_function("lines naive", |b| {
        b.iter(|| {
            v.clear();
            let lines = lines(&string);
            v.extend(lines);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
