#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        pp: [usize; n]
    };
    let v = pp.into_iter().fold(vec![0; 3], |mut v, p| {
        v[if p <= a {
            0
        } else if p <= b {
            1
        } else {
            2
        }] += 1;
        v
    });
    println!("{}", v.into_iter().min().unwrap());
}
