#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    if aa[0] > 0 {
        println!("0");
        return;
    }
    let mut rs = 1;
    let mut vv = [0usize; 3];
    for a in aa {
        let kouho = vv.iter().positions(|&v| v == a).collect_vec();
        rs = rs * kouho.len() % MOD;
        if let Some(&i) = kouho.first() {
            vv[i] += 1;
        }
    }
    println!("{rs}");
}
