#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: isize,
        k: isize,
    };
    n %= k;
    println!("{}", n.min((n - k).abs()));
}
