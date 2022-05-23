#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut aa: [isize; n],
        q: usize,
        bb: [isize; q]
    };
    aa.sort_unstable();
    aa.dedup();
    for b in bb {
        let i = aa.partition_point(|&a| a < b);
        let mut min = std::isize::MAX;
        if i > 0 {
            min = min.min((aa[i - 1] - b).abs());
        }
        if i < aa.len() {
            min = min.min((aa[i] - b).abs());
        }
        println!("{min}");
    }
}
