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
    let cbrt = n.cbrt();
    let mut cnt = 0;
    for a in 1..=cbrt {
        for b in a..=n {
            let c_max = n / a / b;
            if c_max < b {
                break;
            }
            cnt += c_max - b + 1;
        }
    }
    println!("{}", cnt);
}
