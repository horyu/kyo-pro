#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        // p: usize,
        // q: usize,
        // r: usize,
        // nml: [usize; 3],
        pqr: [usize; 3]
    };
    let max = pqr
        .iter()
        .permutations(3)
        .map(|arr| (n / arr[0]) * (m / arr[1]) * (l / arr[2]))
        .max()
        .unwrap();
    println!("{}", max);
}
