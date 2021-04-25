#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    println!("{}", ["No", "Yes"][(a * a + b * b < c * c) as usize]);
}
