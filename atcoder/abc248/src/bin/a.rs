#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s: Chars
    };
    let mut hs: HashSet<_> = ('0'..='9').collect();
    for c in s {
        hs.remove(&c);
    }
    let rs = hs.iter().next().unwrap();
    println!("{rs}");
}
