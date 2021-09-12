#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxyy: [(usize, usize); n]
    };
    let mut hm = HashMap::new();
    for (x, y) in xxyy {
        hm.entry(x).or_insert_with(HashSet::new).insert(y);
    }
    hm.retain(|_k, hs| hs.len() >= 2);
    let mut rs = 0;
    for (&a, &b) in hm.keys().into_iter().tuple_combinations() {
        let points = hm
            .get(&a)
            .unwrap()
            .intersection(hm.get(&b).unwrap())
            .into_iter()
            .count();
        if points > 0 {
            rs += points * (points - 1) / 2;
        }
    }
    println!("{}", rs);
}
