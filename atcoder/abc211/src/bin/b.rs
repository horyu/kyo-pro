#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut ss: [String; 4]
    };
    ss.sort_unstable();
    ss.dedup();
    println!("{}", ["No", "Yes"][(ss.len() == 4) as usize]);
}
