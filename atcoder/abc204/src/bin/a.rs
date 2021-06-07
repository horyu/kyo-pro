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
    };
    println!(
        "{}",
        if x == y {
            x
        } else {
            *[0, 1, 2]
                .iter()
                .filter(|&&i| (i != x) && (i != y))
                .collect_vec()[0]
        }
    );
}
