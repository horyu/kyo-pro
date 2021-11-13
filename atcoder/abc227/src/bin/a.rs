#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: usize,
    };
    let mut rs = (a + k - 1) % n;
    if rs == 0 {
        rs = n;
    }
    println!("{}", rs);
}
