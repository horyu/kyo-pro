#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        ppp: [[usize; 3]; n]
    };
    let pp = ppp
        .into_iter()
        .map(|pp| pp.iter().sum::<usize>())
        .collect_vec();
    let mut scores = pp.clone();
    scores.sort_unstable();
    scores.reverse();
    let kp = scores[k - 1];
    for p in pp {
        if p + 300 >= kp {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
