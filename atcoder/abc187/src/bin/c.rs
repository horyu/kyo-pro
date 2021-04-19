#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        ss: [String; n]
    };
    let mut hs1 = HashSet::new();
    let mut hs2 = HashSet::new();
    for s in ss {
        if s.starts_with('!') {
            hs1.insert(s);
        } else {
            hs2.insert(s);
        }
    }
    for s in hs2 {
        if hs1.contains(&format!("!{}", s)) {
            println!("{}", s);
            return;
        }
    }
    println!("satisfiable");
}
