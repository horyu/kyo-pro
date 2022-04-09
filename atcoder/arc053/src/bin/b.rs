#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        cc: Chars
    };
    let mut hm = HashMap::new();
    for c in cc {
        *hm.entry(c).or_insert(0) += 1;
    }
    let (evens, odds) = hm
        .values()
        .fold((0, 0), |acc, v| (acc.0 + v / 2, acc.1 + v % 2));
    let rs = if odds == 0 {
        evens * 2
    } else {
        1 + evens / odds * 2
    };
    println!("{rs}");
}
