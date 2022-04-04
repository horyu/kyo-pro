#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mm: [usize; n]
    };
    let rs = mm
        .into_iter()
        .fold(0, |acc, m| acc + 80usize.saturating_sub(m));
    println!("{}", rs);
}
