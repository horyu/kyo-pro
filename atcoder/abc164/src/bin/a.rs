#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: usize,
        w: usize,
    };
    println!("{}", ["unsafe", "safe"][(s > w) as usize]);
}
