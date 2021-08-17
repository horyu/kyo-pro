#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: usize,
        x: usize,
        y: usize,
    };
    let rs = if n <= a { x * n } else { x * a + y * (n - a) };
    println!("{}", rs);
}
