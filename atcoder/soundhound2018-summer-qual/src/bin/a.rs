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
        if a + b == 15 {
            '+'
        } else if a * b == 15 {
            '*'
        } else {
            'x'
        }
    );
}
