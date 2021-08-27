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
    let mut rs = 0;
    while 2usize.pow(rs) <= n {
        rs += 1;
    }
    println!("{}", rs - 1);
}
