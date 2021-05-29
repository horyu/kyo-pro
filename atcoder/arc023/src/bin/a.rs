#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut y: usize,
        mut m: usize,
        d: usize,
    };
    if m <= 2 {
        m += 12;
        y -= 1;
    }
    println!(
        "{}",
        735369 - (365 * y + y / 4 - y / 100 + y / 400 + (306 * (m + 1)) / 10 + d - 429)
    );
}
