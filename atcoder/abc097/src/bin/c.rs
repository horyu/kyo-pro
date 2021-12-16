#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
        k: usize
    };
    let mut hs = HashSet::new();
    for i in 0..s.len() {
        for j in i..(s.len().min(i + k)) {
            hs.insert(&s[i..=j]);
        }
    }
    println!("{}", hs.into_iter().sorted().nth(k - 1).unwrap());
}
