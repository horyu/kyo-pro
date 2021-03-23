#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut k: usize,
        a: usize,
        b: usize,
    };
    let ori_k = k;
    while k <= b {
        if k >= a {
            println!("OK");
            return;
        }
        k += ori_k;
    }
    println!("NG");
}
