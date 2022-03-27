#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        r: usize,
        c: usize,
        d: usize,
        aaa: [[usize; c]; r]
    };
    let mut rs = 0;
    for (i, aa) in aaa.into_iter().enumerate() {
        if i > d {
            break;
        }
        for (j, a) in aa.into_iter().enumerate() {
            if i + j > d {
                break;
            }
            if (i + j) % 2 == d % 2 {
                rs = rs.max(a);
            }
        }
    }

    println!("{rs}");
}
