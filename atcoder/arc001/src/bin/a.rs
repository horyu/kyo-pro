#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use itertools::MinMaxResult::{MinMax, NoElements, OneElement};
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        cc: Chars,
    };
    let mut hm = HashMap::new();
    for &c in ['1', '2', '3', '4'].iter() {
        hm.insert(c, 0);
    }
    for c in cc {
        *hm.entry(c).or_insert(0) += 1
    }
    match hm.values().minmax() {
        NoElements => panic!(),
        OneElement(i) => println!("{} {}", i, i),
        MinMax(i, j) => println!("{} {}", j, i),
    }
}
