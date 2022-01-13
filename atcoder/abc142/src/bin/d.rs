#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let adivs = divisors(a);
    let bdivs = divisors(b);
    let rs = adivs.intersection(&bdivs).count();
    println!("{}", rs);
}

fn divisors(mut n: usize) -> HashSet<usize> {
    let mut hs = HashSet::new();
    hs.insert(1);
    if n <= 1 {
        return hs;
    }
    while n % 2 == 0 {
        n /= 2;
        hs.insert(2);
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            n /= i;
            hs.insert(i);
        }
        i += 2;
    }
    if n != 1 {
        hs.insert(n);
    }
    hs
}
