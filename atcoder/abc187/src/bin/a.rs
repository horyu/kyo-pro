#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: String,
        b: String,
    };
    fn to_num(s: &str) -> u32 {
        s.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
    }
    println!("{}", to_num(&a).max(to_num(&b)));
}
