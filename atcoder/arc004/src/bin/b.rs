#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        dd: [isize; n]
    };
    let sum = dd.iter().sum::<isize>();
    let max = *dd.iter().max().unwrap();
    let init = sum - max;
    println!("{}\n{}", sum, (max - init).max(0));
}
