#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        vv: [usize; n]
    };
    let half_n = n / 2;

    let mut evens = HashMap::new();
    let mut odds = HashMap::new();
    for (vx, vy) in vv.into_iter().tuples() {
        *evens.entry(vx).or_insert(0) += 1;
        *odds.entry(vy).or_insert(0) += 1;
    }
    let es = evens.iter().sorted_by_key(|kv| kv.1).rev().collect_vec();
    let os = odds.iter().sorted_by_key(|kv| kv.1).rev().collect_vec();
    let rs = es
        .iter()
        .take(2)
        .cartesian_product(os.iter().take(2))
        .map(|(e, o)| {
            if e.0 == o.0 {
                // 片方全交換
                half_n + (half_n - e.1.max(o.1))
            } else {
                (half_n - e.1) + (half_n - o.1)
            }
        })
        .min()
        .unwrap();
    println!("{}", rs);
}
