#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
    };
    let rs = if (h == 1) || (w == 1) {
        1
    } else if (h * w).is_odd() {
        (h * w) / 2 + 1
    } else {
        (h * w) / 2
    };
    println!("{}", rs);
}
