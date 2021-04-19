#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aabb: [(usize, usize); n],
    };
    aabb.sort_unstable_by_key(|(a, b)| 2 * a + b);
    let mut aa = aabb.iter().fold(0, |acc, (a, _b)| acc + a);
    let mut bb = 0;
    for (i, (a, b)) in aabb.into_iter().rev().enumerate() {
        aa -= a;
        bb += a + b;
        if aa < bb {
            println!("{}", i + 1);
            return;
        }
    }
}
