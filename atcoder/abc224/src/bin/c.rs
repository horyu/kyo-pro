#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(isize, isize); n]
    };
    let cnt = xxyy
        .into_iter()
        .combinations(3)
        .filter(|vv| {
            let (x0, y0) = vv[0];
            let (x1, y1) = vv[1];
            let (x2, y2) = vv[2];
            // 面積公式を使う
            ((x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0)).abs() != 0
        })
        .count();
    println!("{}", cnt);
}
