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
    };
    let mut rs = 0;
    if x <= 3 {
        rs += (4 - x) * 100000;
    }
    if y <= 3 {
        rs += (4 - y) * 100000;
    }
    if (x == 1) && (y == 1) {
        rs += 400000;
    }
    println!("{}", rs);
}
