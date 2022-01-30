#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: u128
    };
    let vv = (0u128..1000).map(|i| i.pow(5u32)).collect_vec();
    for i in 1..1000 {
        for j in 0..i {
            if vv[i] - vv[j] == x {
                println!("{i} {j}");
                return;
            }
            if vv[i] + vv[j] == x {
                println!("{i} -{j}");
                return;
            }
        }
    }
}
