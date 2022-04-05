#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        bbb: [Chars; n]
    };
    let mut bbb = bbb
        .into_iter()
        .map(|bb| {
            bb.into_iter()
                .map(|b| b.to_digit(10).unwrap())
                .collect_vec()
        })
        .collect_vec();
    let mut rs = vec![vec![0; m]; n];
    for i in 0..(n - 1) {
        for j in 0..m {
            let b = bbb[i][j];
            bbb[i][j] -= b;
            rs[i + 1][j] = b;
            if j > 0 {
                bbb[i + 1][j - 1] -= b;
            }
            if j < m - 1 {
                bbb[i + 1][j + 1] -= b;
            }
            if i < n - 2 {
                bbb[i + 2][j] -= b;
            }
        }
    }
    for r in rs {
        println!("{}", r.into_iter().join(""));
    }
}
