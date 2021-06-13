#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    println!("{}", ["No", "Yes"][(aa == (1..=n).collect_vec()) as usize]);
}
