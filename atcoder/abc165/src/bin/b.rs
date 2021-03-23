#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize
    };
    let mut m = 100usize;
    let mut y = 0;
    while m < x {
        y += 1;
        m += m / 100;
    }
    println!("{}", y);
}
