#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [usize; n]
    };
    let mut kouho = HashSet::new();
    for a in 1..200 {
        for b in 1..200 {
            kouho.insert(4 * a * b + 3 * a + 3 * b);
        }
    }
    let rs = ss.into_iter().filter(|s| !kouho.contains(s)).count();
    println!("{}", rs);
}
