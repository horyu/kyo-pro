#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    use std::cmp::Ordering;
    let s = match (n * 108 / 100).cmp(&206) {
        Ordering::Greater => ":(",
        Ordering::Equal => "so-so",
        Ordering::Less => "Yay!",
    };
    println!("{}", s);
}
