#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [i128; n]
    };
    let mut sum = aa.iter().sum::<i128>();
    let mut rs = 0;
    for &a in &aa[0..(n - 1)] {
        sum -= a;
        rs += (a * sum) % 1000000007;
        rs %= 1000000007;
    }
    println!("{}", rs);
}
