#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: u128,
        mut w: u128,
        mut aabb: [(u128, u128); n]
    };
    aabb.sort_unstable_by_key(|ab| ab.0);
    let mut rs = 0u128;
    for (a, b) in aabb.into_iter().rev() {
        if w <= b {
            rs += a * w;
            break;
        }
        w -= b;
        rs += a * b;
    }
    println!("{}", rs);
}
