#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut hs = std::collections::HashSet::new();
    for a in aa {
        if hs.contains(&a) {
            hs.remove(&a);
        } else {
            hs.insert(a);
        }
    }
    println!("{}", hs.len());
}
