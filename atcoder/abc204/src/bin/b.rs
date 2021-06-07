#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let rs = aa
        .into_iter()
        .map(|a| if a > 10 { a - 10 } else { 0 })
        .sum::<usize>();
    println!("{}", rs);
}
