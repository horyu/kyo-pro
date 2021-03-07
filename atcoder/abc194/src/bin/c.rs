#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize,
        aa: [isize; n]
    };

    let mut rs = (n - 1) * aa.iter().fold(0, |acc, a| acc + a.pow(2));
    let mut s = aa[0];
    for a in aa.iter().skip(1) {
        rs -= 2 * a * s;
        s += a;
    }
    println!("{}", rs);
}
