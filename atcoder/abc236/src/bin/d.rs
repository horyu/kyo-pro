#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let mut aaa = vec![vec![0; 2 * n]; 2 * n];
    #[allow(clippy::needless_range_loop)]
    for i in 0..(2 * n - 1) {
        input! {aa: [usize; 2 * n - i - 1]}
        for (j, a) in aa.into_iter().enumerate() {
            aaa[i][i + j + 1] = a;
        }
    }
    let vv = vec![false; 2 * n];
    let rs = dfs(&vv, 0, &aaa);
    println!("{rs}");
}

fn dfs(vv: &[bool], crr: usize, aaa: &[Vec<usize>]) -> usize {
    let unused = vv
        .iter()
        .enumerate()
        .filter_map(|iv| if *iv.1 { None } else { Some(iv.0) })
        .collect_vec();
    if unused.is_empty() {
        return crr;
    }
    let i = unused[0];
    unused[1..]
        .iter()
        .map(|&j| {
            let mut new_vv = vec![];
            new_vv.extend_from_slice(vv);
            new_vv[i] = true;
            new_vv[j] = true;
            dfs(&new_vv, crr ^ aaa[i][j], aaa)
        })
        .max()
        .unwrap()
}
