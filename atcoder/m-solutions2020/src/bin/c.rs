#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    for i in k..n {
        println!("{}", ["No", "Yes"][(aa[i - k] < aa[i]) as usize]);
    }
}
