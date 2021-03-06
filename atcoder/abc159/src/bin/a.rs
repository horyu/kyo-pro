#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    println!(
        "{}",
        n * n.saturating_sub(1) / 2 + m * m.saturating_sub(1) / 2
    );
}
