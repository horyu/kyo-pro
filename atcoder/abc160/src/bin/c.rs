#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize,
        n: usize,
        aa: [usize; n]
    };
    let max_len = aa
        .iter()
        .chain(std::iter::once(&(aa[0] + k)))
        .tuple_windows()
        .map(|(al, ar)| ar - al)
        .max()
        .unwrap();
    println!("{}", k - max_len);
}
