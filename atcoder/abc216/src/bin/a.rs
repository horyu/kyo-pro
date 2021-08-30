#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: String
    };
    let s = s.split('.').collect_vec();
    let x = s[0];
    let y = s[1].parse::<u8>().unwrap();
    println!(
        "{}{}",
        x,
        if y <= 2 {
            "-"
        } else if y <= 6 {
            ""
        } else {
            "+"
        }
    );
}
