#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    const MOD: usize = 1000000007;
    let mut hm = std::collections::HashMap::new();
    for a in aa {
        if n % 2 == a % 2 {
            println!("0");
            return;
        }
        *hm.entry(a).or_insert(0) += 1;
    }
    let mut rs = 1;
    if n % 2 == 0 {
        for i in (1..n).step_by(2) {
            if let Some(&2) = hm.get(&i) {
                rs = rs * 2 % MOD;
            } else {
                println!("0");
                return;
            }
        }
    } else {
        if let Some(&1) = hm.get(&0) {
        } else {
            println!("0");
            return;
        }
        for i in (2..n).step_by(2) {
            if let Some(&2) = hm.get(&i) {
                rs = rs * 2 % MOD;
            } else {
                println!("0");
                return;
            }
        }
    }
    println!("{}", rs);
}
