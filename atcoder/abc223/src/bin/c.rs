#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        aabb: [(f64, f64); n]
    };
    let mut t = aabb.iter().fold(0.0, |acc, (a, b)| acc + a / b) / 2.0;

    let mut rs = 0.0;
    for (a, b) in aabb {
        rs += a.min(t * b);
        t -= t.min(a / b);
    }
    println!("{}", rs);
}
