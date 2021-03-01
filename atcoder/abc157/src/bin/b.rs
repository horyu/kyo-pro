#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        aa: [usize; 9],
        n: usize,
        bb: [usize; n],
    };
    let aa = aa.iter().map(|a| bb.contains(a)).collect_vec();
    for indexes in [
        [0usize, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ]
    .iter()
    {
        if indexes.iter().all(|&index| aa[index]) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
