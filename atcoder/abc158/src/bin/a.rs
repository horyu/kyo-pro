#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut s: Chars
    };
    s.dedup();
    println!("{}", ["No", "Yes"][(s.len() != 1) as usize]);
}
