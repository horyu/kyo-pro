#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    println!(
        "{}",
        if n <= 125 {
            4
        } else if n <= 211 {
            6
        } else {
            8
        }
    );
}
