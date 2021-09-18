#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    println!("{}", ["No", "Yes"][((a <= d) && (c <= b)) as usize]);
}
