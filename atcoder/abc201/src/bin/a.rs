#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut aaa: [isize; 3]
    };
    aaa.sort_unstable();
    println!(
        "{}",
        ["No", "Yes"][(aaa[2] - aaa[1] == aaa[1] - aaa[0]) as usize]
    );
}
