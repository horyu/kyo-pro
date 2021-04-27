#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut ss: [Chars; n]
    };
    ss.sort_unstable_by_key(|s| s.iter().rev().collect::<String>());
    for s in ss {
        println!("{}", s.iter().collect::<String>());
    }
}
