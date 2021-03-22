#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let hs: HashSet<_> = ss.iter().collect();
    println!("{}", hs.len());
}
