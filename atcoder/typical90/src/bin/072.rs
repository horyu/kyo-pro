#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        ccc: [Chars; h]
    };
    let mut ccc = ccc
        .into_iter()
        .map(|cc| cc.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    let mut rs = 0;
    for i in 0..h {
        for j in 0..w {
            rs = rs.max(dfs(&mut ccc, 0, i, j, i, j));
        }
    }
    if rs == 0 {
        println!("-1");
    } else {
        println!("{rs}");
    }
}

fn dfs(ccc: &mut Vec<Vec<bool>>, len: usize, ci: usize, cj: usize, gi: usize, gj: usize) -> usize {
    if ccc[ci][cj] {
        return 0;
    }
    ccc[ci][cj] = true;
    let h = ccc.len();
    let w = ccc[0].len();
    let mut rs = 0;
    if 3 <= len && ci.abs_diff(gi) + cj.abs_diff(gj) == 1 {
        rs = len + 1;
    }
    if 0 < ci {
        rs = rs.max(dfs(ccc, len + 1, ci - 1, cj, gi, gj));
    }
    if ci < h - 1 {
        rs = rs.max(dfs(ccc, len + 1, ci + 1, cj, gi, gj));
    }
    if 0 < cj {
        rs = rs.max(dfs(ccc, len + 1, ci, cj - 1, gi, gj));
    }
    if cj < w - 1 {
        rs = rs.max(dfs(ccc, len + 1, ci, cj + 1, gi, gj));
    }
    ccc[ci][cj] = false;
    rs
}
