#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: isize,
        m: isize,
        aa: [isize; m]
    };
    let rs = n - aa.iter().sum::<isize>();
    println!("{}", if rs < 0 { -1 } else { rs });
}
