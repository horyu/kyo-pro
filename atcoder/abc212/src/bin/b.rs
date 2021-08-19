#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        xx: Chars
    };
    let xx = xx
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect_vec();
    let tf = xx[1..].iter().all(|x| *x == xx[0])
        || xx
            .into_iter()
            .tuple_windows()
            .all(|(a, b)| b == (a + 1) % 10);
    println!("{}", ["Strong", "Weak"][tf as usize]);
}
