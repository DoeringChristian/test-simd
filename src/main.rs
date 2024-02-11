#![feature(portable_simd)]
#![feature(iter_array_chunks)]

use rayon::prelude::*;

use lines::lines_simd;

fn main() {
    let string = std::fs::read_to_string("data/test01.fastq").unwrap();

    let lines = string.lines();

    let lines_simd = lines_simd(&string);

    let lines = lines.collect::<Vec<_>>();
    // let lines_simd = lines_simd.collect::<Vec<_>>();
    // dbg!(lines_simd);

    // assert_eq!(lines, lines_simd);

    // for line in lines_simd {
    //     println!("- {line}");
    // }
}
