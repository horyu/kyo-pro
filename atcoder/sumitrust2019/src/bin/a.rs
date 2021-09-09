#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        m1: usize,
        _d1: usize,
        m2: usize,
        _d2: usize,
    };
    println!("{}", ["0", "1"][(m1 != m2) as usize]);
}
