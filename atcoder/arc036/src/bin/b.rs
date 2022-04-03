#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn main() {
    input! {
        n: usize,
        hh: [usize; n]
    };
    if n == 1 {
        println!("1");
        return;
    }
    let kkcc = hh
        .into_iter()
        .tuple_windows()
        .map(|(hx, hy)| hx.cmp(&hy))
        .group_by(|&k| k)
        .into_iter()
        .map(|(k, g)| (k, g.count()))
        .collect_vec();
    let mut rs = 1;
    if kkcc[0].0.is_ne() {
        rs = rs.max(kkcc[0].1 + 1);
    }
    if kkcc[kkcc.len() - 1].0.is_ne() {
        rs = rs.max(kkcc[kkcc.len() - 1].1 + 1);
    }
    for ((kx, cx), (ky, cy)) in kkcc.into_iter().tuple_windows() {
        if kx.is_le() && ky.is_gt() {
            rs = rs.max(cx + cy + 1);
        }
    }
    println!("{rs}");
}
