#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        bb: [char; 10],
        n: usize,
        mut aa: [String; n]
    };
    aa.sort_by_cached_key(|a| {
        a.chars().fold(0, |acc, c| {
            acc * 10 + bb.iter().position(|&b| b == c).unwrap()
        })
    });
    println!("{}", aa.into_iter().join("\n"));
}
