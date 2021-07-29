#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

fn main() {
    input! {
        n: usize,
        ccc: [[isize; n]; n]
    };
    if n == 1 {
        println!("Yes\n{}\n0", ccc[0][0]);
        return;
    }
    let b1 = (0..n).map(|i| ccc[i][0]).min().unwrap();
    let aa = (0..n).map(|i| ccc[i][0] - b1).collect_vec();
    let bb = (0..n).map(|j| ccc[0][j] - aa[0]).collect_vec();
    if (0..n).all(|i| (0..n).all(|j| aa[i] + bb[j] == ccc[i][j])) {
        println!(
            "Yes\n{}\n{}",
            aa.into_iter().map(|a| a.to_string()).join(" "),
            bb.into_iter().map(|b| b.to_string()).join(" ")
        );
        return;
    }
    println!("No");
}
