#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: u128
    };
    for b in 1..n {
        let bb = 5u128.pow(b as u32);
        if bb >= n {
            break;
        }
        for a in 1..n {
            let wa = 3u128.pow(a as u32) + bb;
            use std::cmp::Ordering;
            match wa.cmp(&n) {
                Ordering::Equal => {
                    println!("{} {}", a, b);
                    return;
                }
                Ordering::Greater => break,
                Ordering::Less => (),
            }
        }
    }
    println!("-1");
}
