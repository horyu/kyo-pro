#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let mut hm = std::collections::HashMap::new();
    for s in ss {
        let count = hm.entry(s).or_insert(0);
        *count += 1;
    }
    println!("{}", hm.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().0);
}
