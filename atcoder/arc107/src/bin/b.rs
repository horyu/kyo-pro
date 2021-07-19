#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize,
        k: isize,
    };
    let mut rs = 0isize;
    let ab_range = if k <= 0 {
        2..=(2 * n + k)
    } else {
        (2 + k)..=(2 * n)
    };
    for ab in ab_range {
        let cd = ab - k;
        let ab_comb = if ab <= n + 1 { ab - 1 } else { 2 * n + 1 - ab };
        let cd_comb = if cd <= n + 1 { cd - 1 } else { 2 * n + 1 - cd };
        rs += ab_comb * cd_comb;
    }
    println!("{}", rs);
}
