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
    let mut rs = vec![];
    for i in n.saturating_sub(200)..=n {
        if g(i) == n {
            rs.push(i);
        }
    }
    println!("{}", rs.len());
    for r in rs {
        println!("{r}");
    }

    // println!("{}", );
}

fn g(mut x: usize) -> usize {
    let mut rs = x;
    while x > 0 {
        rs += x % 10;
        x /= 10;
    }
    rs
}
