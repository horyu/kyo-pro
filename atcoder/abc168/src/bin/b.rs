#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize,
        s: String,
    };
    if s.len() > k {
        println!("{}...", &s[..k]);
    } else {
        println!("{}", s);
    }
}
