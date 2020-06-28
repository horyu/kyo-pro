#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: String,
        k: usize
    };
    let mut hs = std::collections::HashSet::new();
    let len = s.len();
    if k > len {
        println!("0");
        return;
    }
    for left in 0..=(len - k) {
        hs.insert(&s[left..(left + k)]);
    }
    println!("{}", hs.len());
}
