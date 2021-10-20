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
        s: usize,
    };
    const MAX: usize = 1000000000;
    let x = if s == MAX { 1 } else { MAX };
    let aa = vec![s; k];
    let bb = vec![x; n - k];
    println!("{}", aa.iter().chain(bb.iter()).join(" "));
}
