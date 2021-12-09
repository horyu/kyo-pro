#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: Chars,
        b: Chars
    };
    let (mut s, mut b) = if a.len() < b.len() { (a, b) } else { (b, a) };
    s.reverse();
    b.reverse();
    let tf = s
        .into_iter()
        .enumerate()
        .any(|(i, sc)| sc.to_digit(10).unwrap() + b[i].to_digit(10).unwrap() >= 10);
    println!("{}", ["Easy", "Hard"][tf as usize]);
}
