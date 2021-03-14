#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        m: usize,
        h: usize,
    };
    println!("{}", ["No", "Yes"][(h % m == 0) as usize]);
}
