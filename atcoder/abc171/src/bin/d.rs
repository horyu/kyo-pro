#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n],
        q: usize,
        bcbc: [(usize, usize); q],
    };
    use std::collections::HashMap;
    let mut hm = HashMap::new();
    let mut sum = 0;
    for a in aa {
        *hm.entry(a).or_insert(0) += 1;
        sum += a;
    }
    for (b, c) in bcbc {
        if let Some(num) = hm.remove(&b) {
            *hm.entry(c).or_insert(0) += num;
            sum = sum + num * c - num * b
        };
        println!("{}", sum);
    }
}
