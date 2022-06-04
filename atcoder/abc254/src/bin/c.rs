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
    if k == 1 {
        println!("Yes");
        return;
    }
    let mut vvv = vec![vec![]; k];
    for (i, a) in aa.into_iter().enumerate() {
        vvv[i % k].push(a);
    }
    for vv in vvv.iter_mut() {
        vv.sort_unstable();
    }
    for i in 0..(n - 2) {
        let j = i + 1;
        if vvv[i % k][i / k] > vvv[j % k][j / k] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
