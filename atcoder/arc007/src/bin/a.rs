#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: char,
        mut s: Chars
    };
    s.retain(|&c| c != x);
    println!("{}", s.iter().collect::<String>());
}
