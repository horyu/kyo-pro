#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        l: usize,
        r: usize,
        ll: [usize; l],
        rr: [usize; r],
    };
    const SIZE: usize = 41;
    let mut l_counts = [0; SIZE];
    let mut r_counts = [0; SIZE];
    for l in ll {
        l_counts[l] += 1;
    }
    for r in rr {
        r_counts[r] += 1;
    }
    let rs = (0..SIZE).fold(0, |acc, i| acc + l_counts[i].min(r_counts[i]));
    println!("{}", rs);
}
