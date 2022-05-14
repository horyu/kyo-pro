#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        sstt: [(String, usize); n]
    };
    let mut hs = HashSet::new();
    let mut ttii = vec![];
    for (i, (s, t)) in sstt.into_iter().enumerate() {
        if hs.insert(s) {
            ttii.push((t, i + 1));
        }
    }
    let ti = ttii
        .into_iter()
        .min_by(|x, y| y.0.cmp(&x.0).then_with(|| x.1.cmp(&y.1)))
        .unwrap();
    println!("{}", ti.1);
}
