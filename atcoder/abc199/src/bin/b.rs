#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    let a_max = aa.into_iter().max().unwrap();
    let b_min = bb.into_iter().min().unwrap();
    println!("{}", (b_min + 1).saturating_sub(a_max));
}
