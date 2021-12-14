#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    if k == 0 {
        println!("{}", n.pow(2));
        return;
    }

    let mut rs = 0usize;
    for b in (k + 1)..=n {
        // N = pb * r
        let p = n / b;
        let r = n - p * b;

        rs += p * b.saturating_sub(k);
        rs += (r + 1).saturating_sub(k);
    }
    println!("{}", rs);
}
