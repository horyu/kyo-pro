#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: u128,
    };
    let mut rs = 0;
    for i in 1..=n {
        let mut k = i;
        for d in 2u128.. {
            let dd = d.pow(2u32);
            if dd > k {
                break;
            }
            while k % dd == 0 {
                k /= dd;
            }
        }
        rs += (1u128..).take_while(|d| k * d.pow(2u32) <= n).count();
    }
    println!("{rs}");
}
