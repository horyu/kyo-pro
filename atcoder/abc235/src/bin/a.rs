#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{fastout, input, marker::*};
use std::collections::*;

#[fastout]
fn main() {
    input! {
        abc: usize,
    };
    let a = abc / 100;
    let b = abc / 10 % 10;
    let c = abc % 10;
    println!("{}", 111 * (a + b + c));
}
