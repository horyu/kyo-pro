#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let gcd = aa.iter().fold(aa[0], |acc, &a| gcd(acc, a));
    println!("{}", gcd);
}

// https://qiita.com/mhgp/items/711a7bb8e00bff607660
fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = if a < b { (b, a) } else { (a, b) };
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
