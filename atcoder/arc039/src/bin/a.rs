#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
    };
    let a_max = if a < 900 {
        900 + a % 100
    } else if a < 990 {
        990 + a % 10
    } else if a < 999 {
        999
    } else {
        998
    };
    let b_min = if b >= 200 {
        100 + b % 100
    } else if b >= 110 {
        100 + b % 10
    } else if b >= 100 {
        100
    } else {
        101
    };
    println!("{}", (a_max - b).max(a - b_min));
}
