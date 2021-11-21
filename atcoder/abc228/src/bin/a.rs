#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: usize,
        t: usize,
        x: usize,
    };
    let tf = if s < t {
        s <= x && x < t
    } else {
        x < t || s <= x
    };
    println!("{}", ["No", "Yes"][tf as usize]);
}
