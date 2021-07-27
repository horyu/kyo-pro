#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        _m: usize,
        ss: [Chars; n]
    };
    let tmp = ss
        .into_iter()
        .filter(|s| s.iter().filter(|&&c| c == '1').count().is_even())
        .count();
    println!("{}", tmp * (n - tmp));
}
