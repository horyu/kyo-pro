#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::HashMap;

fn main() {
    input! {
        n: usize
    };
    const MOD: usize = 1000000007;
    let mut hm = HashMap::new();
    for i in 1..=n {
        for (k, v) in factors(i) {
            *hm.entry(k).or_insert(0) += v;
        }
    }
    let rs = hm.values().fold(1, |acc, &v| acc * (v + 1) % MOD);
    println!("{}", rs);
}

fn factors(mut n: usize) -> HashMap<usize, usize> {
    let mut hm = HashMap::new();
    if n <= 1 {
        return hm;
    }
    while n % 2 == 0 {
        n /= 2;
        *hm.entry(2).or_insert(0) += 1;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            *hm.entry(i).or_insert(0) += 1;
        }
        i += 2;
    }
    if n != 1 {
        hm.insert(n, 1);
    }
    hm
}
