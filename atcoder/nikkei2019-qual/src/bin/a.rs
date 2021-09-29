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
        b: usize,
    };
    println!("{} {}", a.min(b), (a + b).saturating_sub(n));
}
