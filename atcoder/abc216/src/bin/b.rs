#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    fmt::format,
};

fn main() {
    input! {
        n: usize,
        sstt: [(String, String); n]
    };
    let mut hs = HashSet::new();
    for (s, t) in sstt {
        let st = format!("{}_{}", s, t);
        if hs.contains(&st) {
            println!("Yes");
            return;
        } else {
            hs.insert(st);
        }
    }
    println!("No");
}
