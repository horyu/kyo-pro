#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(Usize1, Usize1); m]
    };
    let mut vvv = vec![vec![]; n];
    for (a, b) in aabb {
        vvv[a].push(b);
        vvv[b].push(a);
    }
    let rs = vvv
        .into_iter()
        .enumerate()
        .filter(|(i, vv)| vv.iter().filter(|&v| v < i).count() == 1)
        .count();
    println!("{rs}");
}
