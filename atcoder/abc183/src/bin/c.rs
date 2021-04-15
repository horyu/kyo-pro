#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        ttt: [[usize; n]; n]
    };
    let rs = (1..n)
        .permutations(n - 1)
        .filter(|v| {
            k == std::iter::once(&0)
                .chain(v.iter())
                .chain(std::iter::once(&0))
                .tuple_windows()
                .map(|(&mae, &ato)| ttt[mae][ato])
                .sum::<usize>()
        })
        .count();
    println!("{}", rs);
}
