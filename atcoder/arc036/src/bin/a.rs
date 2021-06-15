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
        tt: [usize; n]
    };
    let rs = tt.windows(3).position(|vv| vv.iter().sum::<usize>() < k);
    let rs = if let Some(pos) = rs {
        pos as isize + 3
    } else {
        -1
    };
    println!("{}", rs);
}
