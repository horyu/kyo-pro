#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let mut hs = HashSet::new();
    for (i, s) in ss.into_iter().enumerate() {
        if hs.insert(s) {
            println!("{}", i + 1);
        }
    }
}
