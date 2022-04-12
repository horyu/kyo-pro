#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        mut pp: [Usize1; n]
    };
    // 0 1 3 2
    let mut rs = vec![];
    let mut k = 0;
    while k < n - 1 {
        let i = k + pp[k..].iter().position(|&p| p == k).unwrap();
        if i == k {
            println!("-1");
            return;
        }
        for j in ((k + 1)..=i).rev() {
            rs.push(j);
            pp.swap(j - 1, j);
        }
        // eprintln!("{i} {}", pp.iter().join(""));
        for j in k..i {
            if j != pp[j] {
                println!("-1");
                return;
            }
        }
        k = i;
    }
    println!("{}", rs.into_iter().join("\n"));
}
