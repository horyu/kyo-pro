#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize,
    };
    println!("{}", ["No", "Yes"][((x > 0) && (x % 100 == 0)) as usize]);
}
