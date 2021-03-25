#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let mut hs = HashSet::new();
    for _ in 0..k {
        input! {
            d: usize,
            aa: [usize; d]
        };
        for a in aa {
            hs.insert(a);
        }
    }
    println!("{}", n - hs.len());
}
