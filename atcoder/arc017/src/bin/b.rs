#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [usize; n]
    };
    if k == 1 {
        println!("{n}");
        return;
    }

    let mut rs = 0;
    let mut cnt = 0;
    for (ax, ay) in aa.into_iter().tuple_windows() {
        if ax < ay {
            cnt += 1;
        } else {
            cnt = 0;
        }
        if cnt >= (k - 1) {
            rs += 1;
        }
    }
    println!("{rs}");
}
