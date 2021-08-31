#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: usize,
    };
    let mut rs = Vec::new();
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
            rs.push('B');
        } else {
            n -= 1;
            rs.push('A');
        }
    }
    println!("A{}", rs.into_iter().rev().collect::<String>());
}
