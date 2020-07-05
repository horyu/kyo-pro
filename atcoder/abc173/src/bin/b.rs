#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let arr = ["AC", "WA", "TLE", "RE"];
    let mut hm: HashMap<&str, isize> = HashMap::new();
    for s in ss.iter() {
        *hm.entry(s).or_insert(0) += 1;
    }
    for judge in arr.iter() {
        println!("{} x {}", judge, *hm.entry(judge).or_insert(0));
    }
}
