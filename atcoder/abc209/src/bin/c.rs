#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut cc: [usize; n]
    };
    cc.sort_unstable();
    let mut rs = 1;
    for (i, c) in cc.into_iter().enumerate() {
        let mul = c.saturating_sub(i);
        if mul == 0 {
            rs = 0;
            break;
        }
        rs = rs * mul % 1_000_000_007;
    }
    println!("{}", rs);
}
