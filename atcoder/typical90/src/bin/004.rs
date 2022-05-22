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
        aaa: [[usize; w]; h]
    };
    let tate = (0..w)
        .map(|j| (0..h).map(|i| aaa[i][j]).sum::<usize>())
        .collect_vec();
    let yoko = (0..h)
        .map(|i| (0..w).map(|j| aaa[i][j]).sum::<usize>())
        .collect_vec();
    for (i, aa) in aaa.into_iter().enumerate() {
        let rs = aa
            .into_iter()
            .enumerate()
            .map(|(j, a)| yoko[i] + tate[j] - a)
            .join(" ");
        println!("{rs}");
    }
}
