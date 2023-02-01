#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(clippy::uninlined_format_args)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s: String,
    };
    let tf = regex::Regex::new("^[A-Z][1-9]\\d{5}[A-Z]$")
        .unwrap()
        .is_match(s.as_str());
    let rs = ["No", "Yes"][tf as usize];
    println!("{}", rs)
}
