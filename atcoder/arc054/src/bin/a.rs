#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        l: f64,
        x: f64,
        y: f64,
        s: f64,
        d: f64,
    };
    let rs = if d >= s {
        if x >= y {
            (d - s) / (x + y)
        } else {
            ((d - s) / (x + y)).min((l - d + s) / (y - x))
        }
    } else {
        if x >= y {
            (l - s + d) / (x + y)
        } else {
            ((l - s + d) / (x + y)).min((s - d) / (y - x))
        }
    };
    println!("{}", rs);
}
