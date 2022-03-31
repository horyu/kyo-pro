#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut tt: [usize; n]
    };
    let mut cost = 0usize;
    let mut mul = 1usize;
    let mut now = 0usize;

    tt.sort_unstable();
    for (k, g) in tt.into_iter().group_by(|&k| k).into_iter() {
        let n = g.count();
        // 初項now+k, 等差k, n個
        cost += n * (2 * (now + k) + (n - 1) * k) / 2;
        mul = (1..=n).fold(mul, |acc, x| acc * x % (1e9 as usize + 7));
        now += k * n;
    }
    println!("{cost}\n{mul}");
}
