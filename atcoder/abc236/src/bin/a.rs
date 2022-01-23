#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        mut s: Chars,
        a: Usize1,
        b: Usize1,
    };
    s.swap(a, b);
    println!("{}", s.into_iter().join(""));
}
