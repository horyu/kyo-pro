#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n + 1],
        bb: [usize; n]
    };
    let mut sum = 0;
    for i in 0usize..n {
        let b = bb[i];
        if b >= aa[i] + aa[i + 1] {
            sum += aa[i] + aa[i + 1];
            aa[i + 1] = 0;
        } else {
            sum += b;
            aa[i + 1] -= b.checked_sub(aa[i]).unwrap_or(0);
        }
    }
    println!("{}", sum);
}
