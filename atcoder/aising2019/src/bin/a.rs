#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
    };
    println!("{}", (n + 1).saturating_sub(h) * (n + 1).saturating_sub(w));
}
