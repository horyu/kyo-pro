#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: usize,
    };
    if n >= 42 {
        n += 1;
    }
    println!("AGC{:03}", n);
}
