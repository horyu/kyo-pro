#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
    };
    let mut x_indexes = (0..n).collect_vec();
    x_indexes.sort_unstable_by_key(|&i| xxyy[i].0);
    let mut y_indexes = (0..n).collect_vec();
    y_indexes.sort_unstable_by_key(|&i| xxyy[i].1);
    let mut indexes = vec![];
    for i in 0..=1 {
        indexes.push(x_indexes[i]);
        indexes.push(x_indexes[n - 1 - i]);
        indexes.push(y_indexes[i]);
        indexes.push(y_indexes[n - 1 - i]);
    }
    indexes.sort_unstable();
    indexes.dedup();
    let mut vv = indexes
        .into_iter()
        .tuple_combinations()
        .map(|(i, j)| {
            let (xi, yi) = xxyy[i];
            let (xj, yj) = xxyy[j];
            (xi - xj).abs().max((yi - yj).abs())
        })
        .collect_vec();
    vv.sort_unstable();
    println!("{}", vv[vv.len() - 2]);
}
