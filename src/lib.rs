#![feature(portable_simd)]
#![feature(iter_array_chunks)]

use std::simd::cmp::SimdPartialEq;
use std::simd::{self, u8x32, LaneCount, Simd};

const N: usize = 32;

fn find_next_u8simd<const N: usize>(bytes: &[u8], chr: u8) -> Option<usize>
where
    LaneCount<N>: simd::SupportedLaneCount,
{
    let (prefix, middle, suffix) = bytes.as_simd::<N>();

    let mut pos = 0;
    if let Some(pos) = prefix.into_iter().position(|elem| *elem == chr) {
        return Some(pos);
    }

    pos += prefix.len();

    for arr in middle.into_iter() {
        let mask = arr.simd_eq(Simd::<u8, N>::splat(chr));
        if mask.any() {
            return Some(pos + mask.first_set().unwrap());
        }
        pos += N;
    }

    return suffix
        .into_iter()
        .position(|elem| *elem == chr)
        .map(|i| pos + i);
}

pub struct SimdSearch<'a, const N: usize> {
    str: &'a str,
    chr: u8,
}
impl<'a, const N: usize> Iterator for SimdSearch<'a, N>
where
    LaneCount<N>: simd::SupportedLaneCount,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.str.as_bytes();

        let idx = find_next_u8simd::<N>(bytes, self.chr)?;

        self.str = unsafe { std::str::from_utf8_unchecked(&bytes[idx + 1..]) };

        unsafe { Some(std::str::from_utf8_unchecked(&bytes[..idx])) }
    }
}
pub fn lines_simd<'a, const N: usize>(str: &'a str) -> SimdSearch<'a, N> {
    SimdSearch { str, chr: b'\n' }
}

fn find_next_u8(bytes: &[u8], chr: u8) -> Option<usize> {
    bytes.iter().position(|elem| *elem == chr)
}

pub struct Search<'a> {
    str: &'a str,
    chr: u8,
}
impl<'a> Iterator for Search<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.str.as_bytes();

        let idx = find_next_u8(bytes, self.chr)?;

        self.str = unsafe { std::str::from_utf8_unchecked(&bytes[idx + 1..]) };

        unsafe { Some(std::str::from_utf8_unchecked(&bytes[..idx])) }
    }
}

pub fn lines<'a>(str: &'a str) -> Search<'a> {
    Search { str, chr: b'\n' }
}
