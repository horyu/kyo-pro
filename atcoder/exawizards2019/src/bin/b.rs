#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let tf = s
        .into_iter()
        .fold(0i32, |acc, c| acc + if c == 'R' { 1 } else { -1 })
        > 0;
    println!("{}", ["No", "Yes"][tf as usize]);
}
