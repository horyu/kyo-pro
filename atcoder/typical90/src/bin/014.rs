#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    let mut rs = 0;
    for (a, b) in std::iter::zip(aa, bb) {
        rs += a.abs_diff(b);
    }
    println!("{rs}");
}
