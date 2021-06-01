#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        dd: [usize; 7],
        jj: [usize; 7],
    };
    let rs = dd
        .into_iter()
        .zip(jj.into_iter())
        .map(|(d, j)| d.max(j))
        .sum::<usize>();
    println!("{}", rs);
}
