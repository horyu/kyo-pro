#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    println!(
        "{}",
        ["No", "Yes"][((s[2] == s[3]) && (s[4] == s[5])) as usize]
    );
}
