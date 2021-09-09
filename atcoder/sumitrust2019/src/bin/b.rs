#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize
    };
    let opt = ((n * 100 / 108)..std::usize::MAX)
        .take(2)
        .find(|i| i * 108 / 100 == n);
    if let Some(i) = opt {
        println!("{}", i);
    } else {
        println!(":(");
    }
}
