#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut xxyy: [(isize, isize); n]
    };
    xxyy.sort_unstable_by_key(|xy| xy.0);
    // -1 <= (y2-y1)/(x2-x1) <= 1
    let rs = xxyy
        .iter()
        .tuple_combinations()
        .filter(|((x1, y1), (x2, y2))| ((x1 - x2)..=(x2 - x1)).contains(&(y2 - y1)))
        .count();
    println!("{}", rs);
}
