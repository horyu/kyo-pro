#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        _n: usize,
        m: usize,
        ppyy: [(usize, usize); m]
    };
    let mut hm = HashMap::new();
    for (p, y) in &ppyy {
        hm.entry(*p).or_insert(vec![]).push(*y);
    }
    let mut new_hm = HashMap::new();
    for (p, vv) in hm {
        let vv = vv.into_iter().sorted().enumerate();
        for (i, y) in vv {
            new_hm.insert((p, y), i + 1);
        }
    }
    for py in ppyy {
        let i = new_hm.get(&py).unwrap();
        println! {"{:06}{:06}", py.0, i};
    }
}
