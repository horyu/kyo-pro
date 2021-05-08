#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: isize,
        m: usize,
        a: isize,
        b: isize,
        cc: [isize; m]
    };
    for (day, c) in cc.into_iter().enumerate() {
        if n <= a {
            n += b;
        }
        n -= c;
        if n.is_negative() {
            println!("{}", day + 1);
            return;
        }
    }
    println!("complete");
}
