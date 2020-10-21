#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        mut aa: [usize; n]
    };
    let rs = aa
        .into_iter()
        .map(|mut a| {
            let mut num = 0;
            while a % 2 == 0 {
                num += 1;
                a /= 2;
            }
            num
        })
        .sum::<i32>();
    println!("{}", rs);
}
