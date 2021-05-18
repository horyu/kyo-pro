#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ttt: [(f64, f64); n]
    };
    let mut arr = [0; 6];
    for (t1, t2) in ttt {
        if t1 >= 35.0 {
            arr[0] += 1;
        } else if t1 >= 30.0 {
            arr[1] += 1;
        } else if t1 >= 25.0 {
            arr[2] += 1;
        } else if t1 < 0.0 {
            arr[5] += 1;
        }
        if t2 >= 25.0 {
            arr[3] += 1;
        }
        if (t1 >= 0.0) && (t2 < 0.0) {
            arr[4] += 1;
        }
    }
    println!("{}", arr.iter().map(|count| count.to_string()).join(" "));
}
