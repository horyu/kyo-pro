#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    println!("{}", ["NO", "YES"][(n.div_floor(&2) >= k) as usize]);
}
