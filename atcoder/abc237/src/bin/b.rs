#![allow(clippy::many_single_char_names)]
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
    let mut bbb = vec![vec![0; h]; w];
    for (i, aa) in aaa.into_iter().enumerate() {
        for (j, a) in aa.into_iter().enumerate() {
            bbb[j][i] = a;
        }
    }
    for bb in bbb {
        println!("{}", bb.iter().join(" "));
    }
}
