#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize
    };
    let rs = (1..=n)
        .filter(|&i| {
            let mut i = i;
            let mut digit = 0;
            while i != 0 {
                i /= 10;
                digit += 1;
            }
            digit % 2 == 1
        })
        .count();
    println!("{}", rs);
}
