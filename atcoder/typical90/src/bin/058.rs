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
    // https://algo-logic.info/doubling/ タブリング
    const MAX: usize = 1e5 as usize;
    let log_k = (1usize..).find(|i| (1 << i) > k).unwrap();
    let f = |mut x: usize| {
        let mut sum = x;
        while x > 0 {
            sum += x % 10;
            x /= 10;
        }
        sum % MAX
    };
    let mut vvv = vec![vec![0usize; MAX]; log_k];
    for j in 0..MAX {
        vvv[0][j] = f(j);
    }
    for i in 1..log_k {
        for j in 0..MAX {
            vvv[i][j] = vvv[i - 1][vvv[i - 1][j]];
        }
    }
    let mut cur = n;
    for i in 0usize..log_k {
        if (k >> i) & 1 != 0 {
            cur = vvv[i][cur];
        }
    }
    println!("{cur}");
}
