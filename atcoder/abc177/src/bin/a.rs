#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize,
    };
    println!("{}", ["No", "Yes"][(d <= t * s) as usize]);
}
