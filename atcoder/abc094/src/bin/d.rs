#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    aa.sort_unstable();
    let a_max = aa[n - 1];
    let mut rs = 0;
    let mut diff = std::usize::MAX;
    for &a in &aa[0..(n - 1)] {
        // nCr = n! / (r! * (n - r)!)
        if a.max(a_max - a) < diff {
            diff = a.max(a_max - a);
            rs = a;
        }
    }
    println!("{} {}", a_max, rs);
}
