#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [Usize1; n - 1]
    };
    let mut vv = vec![0; n];
    for a in aa {
        vv[a] += 1;
    }
    for v in vv {
        println!("{}", v);
    }
}
