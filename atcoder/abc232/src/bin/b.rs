#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        t: Chars
    };
    let s = s.into_iter().map(|c| c as u8 - b'a').collect_vec();
    let t = t.into_iter().map(|c| c as u8 - b'a').collect_vec();
    let diff = (t[0] + 26 - s[0]) % 26;
    let ss = s.iter().map(|&c| (c + diff) % 26).collect_vec();
    // eprintln!("{}", diff);
    // eprintln!("{}", s.iter().join(" "));
    // eprintln!("{}", ss.iter().join(" "));
    println!("{}", ["No", "Yes"][(ss == t) as usize]);
}
