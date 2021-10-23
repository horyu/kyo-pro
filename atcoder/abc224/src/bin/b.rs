#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        aa: [[usize; w]; h]
    };
    let tf = (0..(h - 1)).all(|i1| {
        let i2 = i1 + 1;
        (0..(w - 1)).all(|j1| {
            let j2 = j1 + 1;
            aa[i1][j1] + aa[i2][j2] <= aa[i2][j1] + aa[i1][j2]
        })
    });
    println!("{}", ["No", "Yes"][tf as usize]);
}
