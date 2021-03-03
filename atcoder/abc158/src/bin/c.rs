#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    // x*0.08 = a
    // x*0.10 = b
    for i in 10..=1250 {
        if (i * 8 / 100 == a) && (i * 10 / 100 == b) {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
