#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    let a = s[0].to_digit(10).unwrap();
    let b = s[2].to_digit(10).unwrap();
    println!("{}", a * b);
}
