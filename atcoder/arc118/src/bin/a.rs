#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
#![feature(int_roundings)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        t: usize,
        n: usize
    };
    let v = (n * 100).div_ceil(t);
    println!("{}", (100 + t) * v / 100 - 1)
}
