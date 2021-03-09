#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [Isize1; n]
    };
    let mut v = vec![0isize; n];
    for &a in &aa {
        v[a as usize] += 1;
    }
    let sum = v.iter().fold(0isize, |acc, &x| acc + x * (x - 1) / 2);
    for &a in &aa {
        let x = v[a as usize];
        println!("{}", sum - x * (x - 1) / 2 + (x - 1) * (x - 2) / 2);
    }
}
