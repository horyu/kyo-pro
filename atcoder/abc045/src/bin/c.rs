#![allow(unused_imports)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let s = s
        .iter()
        .map(|&c| c.to_digit(10).unwrap() as u128)
        .collect_vec();
    let n = s.len();

    let pow2 = (0..(n as u32)).map(|i| 2u128.pow(i)).collect_vec();
    let pow10 = (0..(n as u32)).map(|i| 10u128.pow(i)).collect_vec();

    let mut rs = 0u128;
    rs += (0..n).fold(0u128, |acc, i| {
        acc + s[i] * pow2[i] * pow10[n - 1 - i]
    });
    let mut mul = 0u128;
    for i in 0..(n - 1) {
        mul += pow2[n - 2 - i] * pow10[i];
        rs += s[n - 2 - i] * mul;
    }
    println!("{}", rs);
}
