#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: isize,
        b: isize,
    };
    let mut diff = (a - b).abs();
    let mut rs = diff / 10;
    diff %= 10;
    rs += match diff {
        1 | 5 => 1,
        2 | 4 | 6 | 9 => 2,
        3 | 7 | 8 => 3,
        _ => 0,
    };
    println!("{}", rs);
}
