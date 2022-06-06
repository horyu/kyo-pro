#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        a: u128,
        b: u128,
    };
    let rs = a.lcm(&b);
    if rs <= 1e18 as u128 {
        println!("{rs}");
    } else {
        println!("Large");
    }
}
