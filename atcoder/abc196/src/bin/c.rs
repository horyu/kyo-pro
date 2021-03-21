#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
    };
    let mut rs = 0;
    let mut mul = 10usize;
    let mut x = 1usize;
    while (x * mul + x) <= n {
        rs += 1;
        if (x + 1) == mul {
            mul *= 10;
        }
        x += 1;
    }
    println!("{}", rs);
}
