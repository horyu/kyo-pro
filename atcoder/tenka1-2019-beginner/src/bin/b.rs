#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        s: Chars,
        k: usize
    };
    let mut rs = String::with_capacity(n);
    let ck = s[k - 1];
    for c in s {
        if c == ck {
            rs.push(c);
        } else {
            rs.push('*');
        }
    }
    println!("{rs}");
}
