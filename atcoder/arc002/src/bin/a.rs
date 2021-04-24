#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        y: usize,
    };
    let tf = if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else {
        y % 4 == 0
    };
    println!("{}", ["NO", "YES"][tf as usize]);
}
