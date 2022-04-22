#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(int_log)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
    };
    let mut rs = 0;
    let mut base = 1;
    while base <= n {
        let mut l = base;
        let mut r = l + 1;
        while l <= n {
            rs += if n < r { n - l + 1 } else { r - l };

            l *= 10;
            r *= 10;
        }
        base = base * 10 + 1;
    }
    println!("{rs}");
}
