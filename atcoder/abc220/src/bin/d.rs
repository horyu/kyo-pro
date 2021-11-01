#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut dpp = [0usize; 10];
    dpp[aa[0]] = 1;
    for &a in &aa[1..] {
        let mut dpn = [0usize; 10];
        for (i, p) in dpp.iter().enumerate() {
            dpn[(i + a) % 10] += p;
            dpn[(i * a) % 10] += p;
        }
        for n in dpn.iter_mut() {
            *n %= 998244353;
        }
        dpp = dpn;
    }
    println!("{}", dpp.iter().join("\n"));
}
