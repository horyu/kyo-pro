#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut ccc: [Chars; h]
    };
    for j in 1..w {
        if ccc[0][j - 1] == '#' {
            ccc[0][j] = '#';
        }
    }
    for i in 1..h {
        if ccc[i - 1][0] == '#' {
            ccc[i][0] = '#';
        }
        for j in 1..w {
            if ccc[i][j - 1] == '#' && ccc[i - 1][j] == '#' {
                ccc[i][j] = '#';
            }
        }
    }
    let mut max = 0;
    for (i, cc) in ccc.into_iter().enumerate() {
        for (j, c) in cc.into_iter().enumerate() {
            if c == '.' {
                max = max.max(i + j + 1);
            }
        }
    }
    println!("{}", max);
}
