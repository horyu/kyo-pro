#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        l: usize,
        s: Chars
    };
    let mut current = 1;
    let mut rs = 0;
    for c in s {
        if c == '+' {
            current += 1;
            if current > l {
                current = 1;
                rs += 1;
            }
        } else {
            current = current.saturating_sub(1);
        }
    }
    println!("{}", rs);
}
