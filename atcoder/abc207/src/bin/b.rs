#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let mut water = a;
    let mut red = 0;
    for count in 1..=100000 {
        water += b;
        red += c;
        if water <= red * d {
            println!("{}", count);
            return;
        }
    }
    println!("-1");
}
