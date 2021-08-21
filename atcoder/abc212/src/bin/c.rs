#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut aa: [isize; n],
        mut bb: [isize; m],
    };
    aa.sort_unstable();
    bb.sort_unstable();
    let mut rs = std::isize::MAX;
    let mut i = 0;
    let mut j = 0;
    while (i < n) && (j < m) {
        rs = rs.min((aa[i] - bb[j]).abs());
        if aa[i] > bb[j] {
            j += 1;
        } else {
            i += 1;
        }
    }
    println!("{}", rs);
}
