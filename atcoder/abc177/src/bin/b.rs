#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let tlen = t.len();
    let mut min = tlen;
    for l in 0..=(s.len() - tlen) {
        min = min.min((0..tlen).filter(|&i| s[l + i] != t[i]).count());
    }
    println!("{}", min);
}
