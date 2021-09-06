#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        aabb: [(usize, usize); n],
    };
    let rs = aabb
        .into_iter()
        .filter(|&(a, b)| (a >= h) && (b >= w))
        .count();
    println!("{}", rs);
}
