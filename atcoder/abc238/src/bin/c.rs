#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: u128
    };
    let mut rs = 0;
    let mut x = 10;
    while n >= x {
        let last = x - x / 10;
        rs += (1 + last) * last / 2;
        rs %= 998244353;
        x *= 10;
    }
    x /= 10;
    rs += (1 + (n - (x - 1))) * (n - (x - 1)) / 2;
    rs %= 998244353;
    println!("{rs}");
}
