#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [Chars; n]
    };
    let (r, b) = ss
        .into_iter()
        .flat_map(|s| s)
        .fold((0, 0), |(r, b), s| match s {
            'R' => (r + 1, b),
            'B' => (r, b + 1),
            _ => (r, b),
        });
    use std::cmp::Ordering;
    println!(
        "{}",
        match r.cmp(&b) {
            Ordering::Less => "AOKI",
            Ordering::Equal => "DRAW",
            Ordering::Greater => "TAKAHASHI",
        }
    );
}
