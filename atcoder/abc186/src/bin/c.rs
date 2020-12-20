#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let rs = (1..=n)
        .filter(|&i| {
            let mut p10 = i;
            while p10 > 0 {
                if p10 % 10 == 7 {
                    return false;
                }
                p10 /= 10;
            }
            let mut p8 = i;
            while p8 > 0 {
                if p8 % 8 == 7 {
                    return false;
                }
                p8 /= 8;
            }
            true
        })
        .count();
    println!("{}", rs);
}
