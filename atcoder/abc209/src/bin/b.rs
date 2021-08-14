#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        x: usize,
        aa: [usize; n]
    };
    let sum = aa.into_iter().enumerate().fold(0, |acc, (i, a)| {
        acc + if (i + 1).is_even() { a - 1 } else { a }
    });
    println!("{}", ["No", "Yes"][(sum <= x) as usize]);
}
