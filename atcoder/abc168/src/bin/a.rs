#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n :usize
    };
    println!(
        "{}",
        match n % 10 {
            2 | 4 | 5 | 7 | 9 => "hon",
            0 | 1 | 6 | 8 => "pon",
            _ => "bon",
        }
    );
}
