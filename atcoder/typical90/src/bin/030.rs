#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut xx = vec![0; n + 1];
    let mut rs = 0;
    for i in 2..=n {
        if xx[i] == 0 {
            for j in (i..=n).step_by(i) {
                xx[j] += 1;
                if xx[j] == k {
                    rs += 1;
                }
            }
        }
    }
    println!("{rs}");
}
