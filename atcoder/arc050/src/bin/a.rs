#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: char,
        b: char
    };
    println!(
        "{}",
        ["No", "Yes"][(a.to_lowercase().to_string() == b.to_string()) as usize]
    );
}
