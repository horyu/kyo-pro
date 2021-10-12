#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize,
        n: usize,
    };
    let rs = if n == 1 { 0 } else { k - n };
    println!("{}", rs);
}
