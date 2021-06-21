#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [usize; m]
    };
    let mut viewed = vec![false; n + 1];
    while let Some(a) = aa.pop() {
        if !viewed[a] {
            println!("{}", a);
            viewed[a] = true;
        }
    }
    for (i, a) in viewed.into_iter().enumerate().skip(1) {
        if !a {
            println!("{}", i);
        }
    }
}
