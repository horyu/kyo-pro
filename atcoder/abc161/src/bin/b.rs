#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
    };
    let sum: usize = aa.iter().sum();
    let tf = aa.iter().filter(|&a| 4 * m * a >= sum).count() >= m;
    println!("{}", ["No", "Yes"][tf as usize]);
}
