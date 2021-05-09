#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        m: usize,
        n: usize,
        mut count: usize,
    };
    let mut used = count;
    while used >= m {
        let new = used / m * n;
        count += new;
        used = used % m + new;
    }
    println!("{}", count);
}
