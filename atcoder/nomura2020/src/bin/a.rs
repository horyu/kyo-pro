#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h1: usize,
        m1: usize,
        h2: usize,
        m2: usize,
        k: usize,
    };
    let diff = (h2 * 60 + m2) - (h1 * 60 + m1);
    println!("{}", diff - k);
}
