#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
        x: isize,
        y: isize,
    };
    let mut rs = (x + (b - a).abs() * y).min(x + (b - a + 1).abs() * y);
    if a < b {
        rs = rs.min((2 * (b - a) + 1) * x);
    }
    if a > b {
        rs = rs.min((2 * (a - b - 1) + 1) * x);
    }
    println!("{}", rs);
}
