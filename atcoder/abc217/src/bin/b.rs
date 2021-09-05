#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        ss: [String; 3]
    };
    let arr = ["ABC", "ARC", "AGC", "AHC"];
    let rs = arr
        .iter()
        .filter(|&&s| !ss.contains(&s.to_string()))
        .collect_vec();
    println!("{}", rs[0]);
}
