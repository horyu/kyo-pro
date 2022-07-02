#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        aabb: [(usize, usize); n]
    };
    let mut ab_sum = 0usize;
    let mut rs = std::usize::MAX;
    for (i, (a, b)) in aabb.into_iter().enumerate() {
        ab_sum += a + b;
        rs = rs.min(ab_sum + b * x.saturating_sub(i + 1));
    }
    println!("{rs}");
}
