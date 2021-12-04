#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        d: isize,
        mut llrr: [(isize, isize); n]
    };
    llrr.sort_unstable_by_key(|lr| lr.1);
    let mut rs = 0;
    let mut x = -2e9 as isize;
    for (l, r) in llrr {
        if x + d - 1 < l {
            rs += 1;
            x = r;
        }
    }
    println!("{}", rs);
}
