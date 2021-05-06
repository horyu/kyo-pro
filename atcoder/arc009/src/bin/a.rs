#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n]
    };
    let sum = aabb.into_iter().fold(0, |acc, (a, b)| acc + a * b);
    println!("{}", sum + sum / 20);
}
