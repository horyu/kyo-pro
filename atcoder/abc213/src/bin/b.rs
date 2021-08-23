#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut one = (0, 0);
    let mut two = (0, 0);
    for (i, a) in aa.into_iter().enumerate() {
        if a > one.0 {
            two = one;
            one = (a, i);
        } else if a > two.0 {
            two = (a, i);
        }
    }
    println!("{}", two.1 + 1);
}
