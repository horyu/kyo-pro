#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        pp: [usize; 26]
    };
    let s = pp
        .into_iter()
        .map(|n| (b'a' + n as u8 - 1) as char)
        .collect::<String>();
    println!("{}", s);
}
