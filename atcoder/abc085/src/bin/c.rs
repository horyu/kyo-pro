#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: isize,
        a: isize
    };
    for x in 0..=n {
        let xx = 10000 * x;
        for y in 0..=(n - x) {
            let z = n - x - y;
            let sum = xx + 5000 * y + 1000 * z;
            if sum == a {
                println!("{} {} {}", x, y, z);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}
