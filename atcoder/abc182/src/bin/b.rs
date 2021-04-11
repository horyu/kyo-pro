#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let rs = (2..=1000)
        .max_by_key(|&i| aa.iter().filter(|&&a| a % i == 0).count())
        .unwrap();
    println!("{}", rs);
}
