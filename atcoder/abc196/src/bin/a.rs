#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize
    };
    println!("{}", (a - c).max(a - d).max(b - c).max(b - d));
}
