#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut xx = aa.clone();
    xx.sort_unstable();
    xx.dedup();
    let mut hm = std::collections::HashMap::new();
    for (i, x) in xx.into_iter().enumerate() {
        hm.insert(x, i);
    }
    let s = aa.into_iter().map(|a| hm.get(&a).unwrap()).join("\n");
    println!("{}", s);
}
