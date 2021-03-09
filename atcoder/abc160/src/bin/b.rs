#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize
    };
    println!("{}", x / 500 * 1000 + x % 500 / 5 * 5);
}
