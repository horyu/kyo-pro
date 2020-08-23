#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        aabb: [(usize, usize); m]
    };
    let mut first = HashSet::new();
    let mut second = HashSet::new();
    for (a, b) in aabb {
        if a == 1 {
            first.insert(b);
        }
        if b == n {
            second.insert(a);
        }
    }
    println!(
        "{}POSSIBLE",
        ["", "IM"][(first.intersection(&second).count() == 0) as usize]
    );
}
