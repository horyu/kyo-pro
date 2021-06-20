#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize,
        y: usize,
        k: usize,
    };
    let rs = if k > y {
        y + x - (k - y)
    } else {
        x + k
    };
    println!("{}", rs);
}
