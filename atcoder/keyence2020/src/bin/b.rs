#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        xxll: [(isize, isize); n]
    };
    let mut ttss = vec![];
    for (x, l) in xxll {
        ttss.push((x + l, x - l));
    }
    ttss.sort_unstable();
    let mut rs = 0;
    let mut cur = std::isize::MIN;
    for (t, s) in ttss {
        if cur <= s {
            rs += 1;
            cur = t;
        }
    }
    println!("{rs}");
}
