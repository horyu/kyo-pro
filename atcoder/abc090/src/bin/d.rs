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
        let m = (n + 1) / b;
        rs += (b - k) * m;

        let l = k + b * m;
        let r = n;
        if l <= r {
            rs += r - l + 1;
        }
    }
    println!("{}", rs);
}
