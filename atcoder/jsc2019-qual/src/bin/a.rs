#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        m: usize,
        d: usize,
    };
    let mut cnt = 0;
    for m in 4..=m {
        for d in 22..=d {
            let (d10, d1) = d.div_mod_floor(&10);
            if (d1 >= 2) && (m == d10 * d1) {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
