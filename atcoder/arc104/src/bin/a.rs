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
    println!("{} {}", (a + b) / 2, (a - b) / 2);
}
