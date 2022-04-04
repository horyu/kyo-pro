#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        r: usize,
        mut s: Chars
    };
    let mut rs = 0;
    if let Some(rr) = s.iter().rposition(|&c| c == '.') {
        for ll in 0..(rr.saturating_sub(r - 1)) {
            if s[ll] == '.' {
                for i in 0..r {
                    s[ll + i] = 'o';
                }
                rs += 1;
            }
            rs += 1;
        }
        rs += 1;
    }
    println!("{rs}");
}
