#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let mut vv: Vec<Vec<usize>> = vec![];
    for i in 0..n {
        let mut aa = vec![1usize];
        for j in 1..=i {
            if j == i {
                aa.push(1);
            } else {
                aa.push(vv[i - 1][j - 1] + vv[i - 1][j]);
            }
        }
        println!("{}", aa.iter().join(" "));
        vv.push(aa);
    }
}
