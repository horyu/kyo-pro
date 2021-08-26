#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [usize; n],
        mut tt: [usize; n],
    };
    for i in (0..n).chain(0..n) {
        let next = (i + 1) % n;
        tt[next] = tt[next].min(ss[i] + tt[i]);
    }
    println!("{}", tt.iter().join("\n"));
}
