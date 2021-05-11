#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        va: f64,
        vb: f64,
        mut l: f64,
    };
    for _ in 0..n {
        let s = l / va;
        l = vb * s;
    }
    println!("{}", l);
}
