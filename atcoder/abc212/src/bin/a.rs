#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    println!(
        "{}",
        if a > 0 && b == 0 {
            "Gold"
        } else if a == 0 && b > 0 {
            "Silver"
        } else {
            "Alloy"
        }
    );
}
