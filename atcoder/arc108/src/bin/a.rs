#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: i128,
        p: i128,
    };
    // n+m=s, nm=p
    // p/m + m = s
    // mm - sm + p = 0;
    // m = (s +- sqrt(ss - 4p))/2
    let d = s.pow(2u32) - 4 * p;
    if d.is_negative() || (d.sqrt().pow(2u32) != d) {
        println!("No");
        return;
    }
    let sqrt = d.sqrt();
    if (s - sqrt) <= 0 || (s - sqrt).is_odd() {
        println!("No");
        return;
    }
    println!("Yes");
}
