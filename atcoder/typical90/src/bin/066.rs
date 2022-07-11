#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        llrr: [(usize, usize); n],
    };
    let mut rs = 0.0;
    for i in 0..(n - 1) {
        let (li, ri) = llrr[i];
        for j in (i + 1)..n {
            let (lj, rj) = llrr[j];
            let mut cnt = 0.0;
            for x in li..=ri {
                for y in lj..=rj {
                    if y < x {
                        cnt += 1.0;
                    }
                }
            }
            rs += cnt / ((ri - li + 1) * (rj - lj + 1)) as f64;
        }
    }
    println!("{rs}");
}
