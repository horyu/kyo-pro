#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: u128,
        l: u128,
        r: u128,
    };
    // n1010101
    // x0000001
    // x00001**
    // x001****
    // x1******
    // l0011010
    // r1011000
    let mut rs = 0;
    for i in 0..70 {
        let base = 1u128 << i;
        let max = std::u128::MAX >> (127 - i);
        if base & n == 0 || max < l {
            continue;
        }
        if r < base {
            break;
        }
        rs += base;
        if base < l && l <= max {
            rs -= l ^ base;
        }
        if r < max {
            rs -= max - r;
        }
        // dbg!(i, base, max, l, r);
    }
    println!("{rs}");
}
