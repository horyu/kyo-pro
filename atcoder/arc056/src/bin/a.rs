#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
        l: usize,
    };
    let rs = (a * k).min(k % l * a + k / l * b).min(k.div_ceil(&l) * b);
    println!("{}", rs);
}
