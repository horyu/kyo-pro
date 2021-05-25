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
    use std::cmp::Ordering::*;
    println!(
        "{}",
        match a.abs().cmp(&b.abs()) {
            Less => "Ant",
            Equal => "Draw",
            Greater => "Bug",
        }
    );
}
