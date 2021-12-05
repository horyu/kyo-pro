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
    for i in 0..h {
        for j in 0..w {
            if ccc[i][j] != '.' {
                continue;
            }
            let mut v = vec![];
            if 0 < i {
                v.push(ccc[i - 1][j]);
            };
            if i < h - 1 {
                v.push(ccc[i + 1][j]);
            }
            if 0 < j {
                v.push(ccc[i][j - 1]);
            }
            if j < w - 1 {
                v.push(ccc[i][j + 1]);
            }
            ccc[i][j] = *['1', '2', '3', '4', '5']
                .iter()
                .find(|&c| !v.contains(c))
                .unwrap();
        }
    }
    for cc in ccc {
        println!("{}", cc.into_iter().collect::<String>());
    }
}
