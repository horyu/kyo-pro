#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        ss: [String; n],
        m: usize,
        tt: [String; m],
    };
    let mut hm = std::collections::HashMap::new();
    for s in ss {
        *hm.entry(s).or_insert(0) += 1;
    }
    for t in tt {
        *hm.entry(t).or_insert(0) -= 1;
    }
    let rs = std::cmp::max(0, *hm.values().max().unwrap());
    println!("{}", rs);
}
