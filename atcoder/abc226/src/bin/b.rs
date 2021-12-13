#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    let mut hs = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            aa: [usize; l]
        };
        hs.insert(aa);
    }
    println!("{}", hs.len());
}
