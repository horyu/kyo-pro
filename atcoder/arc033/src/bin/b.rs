#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        na: usize,
        nb: usize,
        aa: [usize; na],
        bb: [usize; nb],
    };
    let sa: HashSet<_> = aa.iter().collect();
    let sb: HashSet<_> = bb.iter().collect();
    let rs = sa.intersection(&sb).count() as f64 / sa.union(&sb).count() as f64;
    println!("{}", rs);
}
