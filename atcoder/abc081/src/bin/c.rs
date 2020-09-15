#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    let mut hm = std::collections::HashMap::new();
    for a in aa {
        *hm.entry(a).or_insert(0) += 1;
    }

    let mut v = hm.values().into_iter().map(|&i| i).collect::<Vec<i32>>();
    v.sort();
    if v.len() <= k {
        println!("0");
        return;
    }

    println!("{}", v[0..(v.len() - k)].into_iter().sum::<i32>());
}
