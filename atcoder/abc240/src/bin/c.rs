#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        aabb: [(usize, usize); n]
    };
    let mut hs = HashSet::new();
    hs.insert(0usize);
    for (a, b) in aabb {
        let mut new_hs = HashSet::new();
        for v in hs {
            new_hs.insert(v + a);
            new_hs.insert(v + b);
        }
        hs = new_hs;
    }
    let tf = hs.contains(&x);
    println!("{}", ["No", "Yes"][tf as usize]);
}
