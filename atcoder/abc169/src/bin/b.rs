#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [u128; n]
    };
    aa.sort_unstable();
    let mut sum = 1u128;
    for a in aa {
        let (tmp, tf) = sum.overflowing_mul(a);
        if tf || tmp > (1e18 as u128) {
            dbg!(tmp, tf);
            println!("-1");
            return;
        }
        sum = tmp;
    }
    println!("{}", sum);
}
