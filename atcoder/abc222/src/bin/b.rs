#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        p: usize,
        aa: [usize; n]
    };
    let cnt = aa.into_iter().filter(|&a| a < p).count();
    println!("{}", cnt);
}
