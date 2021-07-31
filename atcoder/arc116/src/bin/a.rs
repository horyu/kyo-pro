#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        t: usize,
        cc: [usize; t]
    };
    for mut c in cc {
        let s = if c.is_odd() {
            "Odd"
        } else {
            c /= 2;
            if c != 1 && c.is_even() {
                "Even"
            } else {
                "Same"
            }
        };
        println!("{}", s);
    }
}
