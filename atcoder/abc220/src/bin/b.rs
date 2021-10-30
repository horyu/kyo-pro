#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: u32,
        a: String,
        b: String,
    };
    let a = usize::from_str_radix(&a, k).unwrap();
    let b = usize::from_str_radix(&b, k).unwrap();
    println!("{}", a * b);
}
