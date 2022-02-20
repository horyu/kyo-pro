#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let hs: HashSet<usize> = aa.into_iter().collect();
    println!("{}", hs.len());
}
