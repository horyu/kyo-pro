#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        n: usize,
    };
    let f = |x: usize| a * x / b - a * (x / b);
    println!("{}", f(n.min(b - 1)));
}
