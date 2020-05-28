#![allow(unused_imports)]
// use itertools::Itertools;
use ascii::{AsciiChar, AsciiString};
use proconio::{input, marker::*};

fn main() {
    input! {
        mut s: AsciiString
    };
    s.make_ascii_lowercase();
    s.as_mut_slice()[0].make_ascii_uppercase();
    println!("{}", s.as_str());
}
