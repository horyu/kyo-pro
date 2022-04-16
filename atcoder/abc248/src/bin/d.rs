#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n],
        q: usize,
        llrrxx: [(usize, usize, Usize1); q]
    };
    let mut vvv = vec![vec![]; n];
    for (i, a) in aa.into_iter().enumerate() {
        vvv[a].push(i);
    }
    for (l, r, x) in llrrxx {
        let ri = vvv[x].partition_point(|&v| v < r);
        let li = vvv[x].partition_point(|&v| v < l - 1);
        let rs = ri.saturating_sub(li);
        println!("{rs}");
    }
}
