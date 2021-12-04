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
    let mut t = String::new();
    for _ in 0..=10 {
        t.push_str("oxx");
    }
    println!("{}", ["No", "Yes"][t.contains(&s) as usize]);
}
