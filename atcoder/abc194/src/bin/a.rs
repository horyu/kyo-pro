#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ab = a + b;
    println!(
        "{}",
        if ab >= 15 && b >= 8 {
            1
        } else if ab >= 10 && b >= 3 {
            2
        } else if ab >= 3 {
            3
        } else {
            4
        }
    );
}
