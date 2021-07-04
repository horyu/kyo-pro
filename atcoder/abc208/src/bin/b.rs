#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut p: usize
    };
    let mut count = 0;
    while p > 0 {
        let mut n = 1;
        let mut max = 1;
        while max <= p {
            n += 1;
            max *= n;
        }
        count += 1;
        max /= n;
        p -= max;
    }
    println!("{}", count);
}
