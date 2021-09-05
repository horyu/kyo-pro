#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: String,
        t: String,
    };
    let v1 = vec![s.clone(), t.clone()];
    let mut v2 = vec![s, t];
    v2.sort_unstable();
    println!("{}", ["No", "Yes"][(v1 == v2) as usize]);
}
