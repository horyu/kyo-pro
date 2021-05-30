#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let rs = (1..=n).fold(0, |acc, i| acc + 100 * i * k + (1..=k).sum::<usize>());
    println!("{}", rs);
}
