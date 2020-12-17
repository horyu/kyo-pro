#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: Chars
    };
    s.sort();
    println!(
        "{}",
        ["No", "Yes"][((s[0] == s[1]) && (s[1] != s[2]) && (s[2] == s[3])) as usize]
    );
}
