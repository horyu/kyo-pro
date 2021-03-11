#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: u8,
        y: u8,
        z: u8,
    };
    println!("{} {} {}", z, x, y);
}
