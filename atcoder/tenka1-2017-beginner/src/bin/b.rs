#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aabb: [(usize, usize); n]
    };
    let (a, b) = aabb.into_iter().max_by_key(|ab| ab.0).unwrap();
    println!("{}", a + b);
}
