#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        pp: [Usize1; n]
    };
    let mut rs = vec![0; n];
    for (i, p) in pp.into_iter().enumerate() {
        rs[p] = i + 1;
    }
    println!("{}", rs.into_iter().map(|r| r.to_string()).join(" "));
}
