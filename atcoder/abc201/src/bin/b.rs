#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut sstt: [(String, isize); n]
    };
    sstt.sort_unstable_by_key(|st| st.1);
    sstt.reverse();
    println!("{}", sstt[1].0);
}
