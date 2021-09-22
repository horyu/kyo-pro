#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };
    // x:y = z:rs
    // rs = y * z / x
    println!(
        "{}",
        if (y * z).is_multiple_of(&x) {
            y * z / x - 1
        } else {
            y * z / x
        }
    );
}
