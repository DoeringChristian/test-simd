use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lines::{lines, lines_simd};

macro_rules! simd {
    ($group:expr, $string:expr, $size:literal) => {
        let mut v = vec![];
        $group.bench_function(concat!("simd", $size), |b| {
            b.iter(|| {
                v.clear();
                let lines = lines_simd::<$size>(&$string);
                v.extend(lines);
            })
        });
    };
    ($group:expr, $string:expr, $($sizes:literal),*) => {
        $(simd!($group, $string, $sizes);)*
    };
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let string = std::fs::read_to_string("data/9_Swamp_S2B_ITS2_2019_minq7.fastq").unwrap();

    let mut group = c.benchmark_group("lines");

    let mut v = vec![];
    group.bench_function("rust", |b| {
        b.iter(|| {
            v.clear();
            let lines = string.lines();
            v.extend(lines);
        })
    });

    let mut v = vec![];
    group.bench_function("naive", |b| {
        b.iter(|| {
            v.clear();
            let lines = lines(&string);
            v.extend(lines);
        })
    });

    simd!(group, string, 2, 4, 8, 16, 32, 64);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
