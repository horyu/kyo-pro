#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        sstt: [(String, usize); n],
        x: String
    };
    let l = sstt.iter().position(|st| st.0 == x).unwrap() + 1;
    let rs = sstt[l..].iter().map(|st| st.1).sum::<usize>();
    println!("{}", rs);
}
