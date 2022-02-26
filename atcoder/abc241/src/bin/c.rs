#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    };
    let bbb = ss
        .into_iter()
        .map(|s| s.into_iter().map(|c| c == '#').collect_vec())
        .collect_vec();
    for i in 0..n {
        for j in 0..=(n - 6) {
            if bbb[i][j..(j + 6)].iter().filter(|&&tf| tf).count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..=(n - 6) {
        for j in 0..n {
            if (i..(i + 6)).filter(|&i| bbb[i][j]).count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..=(n - 6) {
        for j in 0..=(n - 6) {
            if (0..6).filter(|&k| bbb[i + k][j + k]).count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }
    for i in 0..=(n - 6) {
        for j in 5..n {
            if (0..6).filter(|&k| bbb[i + k][j - k]).count() >= 4 {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
