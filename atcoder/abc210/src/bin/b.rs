#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };
    let pos = s.into_iter().position(|c| c == '1').unwrap();
    println!("{}", ["Aoki", "Takahashi"][pos.is_even() as usize]);
}
