#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        xx: [usize; n]
    };
    let mut rs = 0;
    for (xi, xj) in xx.into_iter().tuple_windows() {
        rs += b.min(a * (xj - xi));
    }
    println!("{}", rs);
}
