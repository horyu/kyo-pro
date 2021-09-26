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
    let mut tf = s.starts_with("keyence") || s.ends_with("keyence");
    if !tf {
        tf = [
            ("k", "eyence"),
            ("ke", "yence"),
            ("key", "ence"),
            ("keye", "nce"),
            ("keyen", "ce"),
            ("keyenc", "e"),
        ]
        .iter()
        .any(|(l, r)| s.starts_with(l) && s.ends_with(r));
    }
    println!("{}", ["NO", "YES"][tf as usize]);
}
