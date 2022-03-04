#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let tf = aa.into_iter().all(|a| a % 2 == 0);
    println!("{}", ["first" ,"second"][tf as usize]);
}
