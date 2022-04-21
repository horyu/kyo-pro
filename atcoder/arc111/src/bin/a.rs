#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    // https://docs.rs/modpow/latest/src/modpow/lib.rs.html#21
    let rs = modpow(10, n, m * m) / m % m;
    println!("{rs}");
}

fn modpow(mut base: usize, mut exp: usize, n: usize) -> usize {
    let mut res = 1;
    base %= n;
    loop {
        if exp % 2 == 1 {
            res *= base;
            res %= n;
        }
        if exp == 1 {
            return res;
        }

        exp /= 2;
        base *= base;
        base %= n;
    }
}
