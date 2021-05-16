#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ww: [String; n],
    };
    if n > 1 {
        let mut pre = &ww[0];
        let mut hs = HashSet::new();
        for (i, w) in ww.iter().skip(1).enumerate() {
            hs.insert(pre);
            if hs.contains(w) || !w.starts_with(pre.chars().last().unwrap()) {
                println!("{}", ["LOSE", "WIN"][(i % 2 == 0) as usize]);
                return;
            }
            pre = w;
        }
    }
    println!("DRAW");
}
