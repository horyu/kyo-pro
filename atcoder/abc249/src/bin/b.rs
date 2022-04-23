#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s: Chars
    };
    let mut hs = HashSet::new();
    for c in s {
        if !hs.insert(c) {
            println!("No");
            return;
        }
    }
    let tf = ('a'..='z').any(|c| hs.contains(&c)) && ('A'..='Z').any(|c| hs.contains(&c));
    let rs = if tf { "Yes" } else { "No" };
    println!("{rs}");
}
