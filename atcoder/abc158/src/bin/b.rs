#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    };
    println!("{}", n / (a + b) * a + (n % (a + b)).min(a));
}
