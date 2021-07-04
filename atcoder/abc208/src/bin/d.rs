#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    vec,
};
fn main() {
    input! {
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m]
    };
    let mut d = vec![vec![std::usize::MAX; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for (a, b, c) in aabbcc {
        d[a][b] = c;
    }
    let mut sum = 0usize;
    for k in 0..n {
        let mut next = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                next[i][j] = d[i][j].min(d[i][k].saturating_add(d[k][j]));
                if next[i][j] != std::usize::MAX {
                    sum += next[i][j];
                }
            }
        }
        d = next;
    }
    println!("{}", sum);
}
