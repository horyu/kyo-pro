#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        k: usize
    };
    let mut a = 2;
    let mut b = 1;
    for _ in 1..k {
        (a, b) = (a + b, a);
        // eprintln!("{i} {a} {b}");
    }
    println!("{a} {b}");
    // let mut c = 0;
    // while b != 0 {
    //     eprintln!("{c} {a} {b}");
    //     c += 1;
    //     (a, b) = (b, a % b);
    // }
    // eprintln!("{c} {a} {b}")
}
