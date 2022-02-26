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
        aa: [usize; n]
    };
    let mut viewed = vec![std::usize::MAX; n];
    let mut crr = 0;
    let mut vv = vec![crr];
    for i in 1..=k {
        let j = crr % n;
        let a = aa[j];
        if viewed[j] == std::usize::MAX {
            // 初見
            crr += a;
            viewed[j] = i;
            vv.push(crr);
            // eprintln!("{}: {} {}", i, j, crr);
        } else {
            let loop_size = i - viewed[j];
            let loop_sum = *vv.last().unwrap() - vv[viewed[j] - 1];

            let nokori = k + 1 - i;
            crr += (nokori / loop_size) * loop_sum;
            crr += vv[viewed[j] - 1 + (nokori % loop_size)] - vv[viewed[j] - 1];
            // dbg!(vv, viewed, j, a, i);
            // dbg!(loop_size, loop_sum, crr, nokori);
            break;
        }
    }
    println!("{crr}");
}
