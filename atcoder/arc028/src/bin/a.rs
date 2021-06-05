#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: usize,
        a: usize,
        b: usize,
    };
    n %= a + b;
    let rs = if (n == 0) || (n > a) { "Bug" } else { "Ant" };
    println!("{}", rs);
}
