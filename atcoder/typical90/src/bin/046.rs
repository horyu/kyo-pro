#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        xxx: [[usize; n]; 3],
    };
    let mut vvv = [[0usize; 46]; 3];
    for (i, xx) in xxx.into_iter().enumerate() {
        for x in xx {
            vvv[i][x % 46] += 1;
        }
    }
    let mut rs = 0;
    for i in 0..46 {
        for j in 0..46 {
            let k = (46 - (i + j) % 46) % 46;
            rs += vvv[0][i] * vvv[1][j] * vvv[2][k];
        }
    }
    println!("{rs}");
}
