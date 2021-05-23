#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        abc: [usize; 3]
    };
    println!("{}", abc.into_iter().map(|x| 7 - x).sum::<usize>());
}
