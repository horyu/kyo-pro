#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut aabb: [(isize, isize); n]
    };
    aabb.sort_unstable_by_key(|ab| ab.0 + ab.1);
    let mut x = 0;
    let mut y = 0;
    while let Some(ab) = aabb.pop() {
        x += ab.0;
        if let Some(ab) = aabb.pop() {
            y += ab.1;
        }
    }
    println!("{}", x - y);
}
