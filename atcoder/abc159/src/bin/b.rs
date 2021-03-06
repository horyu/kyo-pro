#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    let n = s.len();
    println!(
        "{}",
        ["No", "Yes"][(is_kaibun(&s) && is_kaibun(&s[((n + 3) / 2 - 1)..])) as usize]
    );
}

fn is_kaibun(s: &[char]) -> bool {
    let len = s.len();
    if len == 1 {
        return true;
    }
    (0..(len / 2)).all(|i| s[i] == s[len - 1 - i])
}
