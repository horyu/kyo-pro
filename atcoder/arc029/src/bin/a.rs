#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut tt: [usize; n]
    };
    tt.sort_unstable();
    let rs = match n {
        1 => tt[0],
        2 => tt[1],
        3 => tt[2].max(tt[0] + tt[1]),
        _ => ((tt[0] + tt[3]).max(tt[1] + tt[2])).min(tt[3].max(tt[0] + tt[1] + tt[2])),
    };
    println!("{}", rs);
}
