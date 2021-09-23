#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize
    };
    let mut rs = 1;
    for i in 2..=(b - a) {
        let l = i * a.div_ceil(&i);
        let r = i * (a.div_ceil(&i) + 1);
        if (a..=b).contains(&l) && (a..=b).contains(&r) {
            rs = i;
        }
    }
    println!("{}", rs);
}
