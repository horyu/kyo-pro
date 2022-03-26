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
        aa: [isize; n + 1],
        mut cc: [isize; n + m + 1],
    };
    let mut bb = vec![0; m + 1];
    for i in (0..=m).rev() {
        bb[i] = cc[i + n] / aa[n];
        for j in 0..=n {
            cc[i + j] -= bb[i] * aa[j];
        }
    }
    let rs = bb.into_iter().join(" ");
    println!("{rs}");
}
