#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aaa: [[usize; n]; n],
        m: usize,
        xxyy: [(Usize1, Usize1); m],
    };
    let mut ng = vec![vec![false; n]; n];
    for (x, y) in xxyy {
        ng[x][y] = true;
        ng[y][x] = true;
    }
    let rs_ott = (0..n)
        .permutations(n)
        .flat_map(|xx| {
            if xx.iter().tuple_windows().any(|(&x, &y)| ng[x][y]) {
                None
            } else {
                let sum = xx
                    .into_iter()
                    .enumerate()
                    .fold(0, |acc, (i, x)| acc + aaa[x][i]);
                Some(sum)
            }
        })
        .min();
    if let Some(rs) = rs_ott {
        println!("{rs}");
    } else {
        println!("-1");
    }
}
