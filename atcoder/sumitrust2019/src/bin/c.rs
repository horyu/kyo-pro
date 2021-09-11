#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize
    };
    let mut l = 100;
    let mut r = 105;
    while x >= l {
        if (l..=r).contains(&x) {
            println!("1");
            return;
        }
        l += 100;
        r += 105;
    }
    println!("0");
}
