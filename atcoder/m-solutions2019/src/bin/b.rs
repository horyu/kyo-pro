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
    let k = s.len();
    let win = s.into_iter().filter(|&c| c == 'o').count();
    let tf = 15 - k + win >= 8;
    println!("{}", ["NO", "YES"][tf as usize]);
}
