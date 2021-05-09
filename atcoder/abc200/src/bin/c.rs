#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut arr = [0usize; 200];
    for a in aa {
        arr[a % 200] += 1;
    }
    let rs = arr
        .iter()
        .map(|&count| count.saturating_sub(1) * count / 2)
        .sum::<usize>();
    println!("{}", rs);
}
