#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        s: Chars
    };

    let mut rs = 0;
    for mut cc in (0..3).map(|_| '0'..='9').multi_cartesian_product() {
        for &c in &s {
            if let Some(&last) = cc.last() {
                if c == last {
                    cc.pop();
                    if cc.is_empty() {
                        rs += 1;
                        break;
                    }
                }
            }
        }
    }
    println!("{rs}");
}
