#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
        bb: [usize; n],
    };
    if n == 1 {
        println!("{}", aa[0] ^ bb[0]);
        return;
    }
    let mut oks = HashSet::new();
    for (i, b) in bb.iter().enumerate() {
        let mut used = vec![false; n];
        let v = aa[0] ^ b;
        used[i] = true;
        let is_ok = (1..n).all(|j| {
            (0..n).any(|k| {
                if !used[k] && (aa[j] ^ bb[k] == v) {
                    used[k] = true;
                    true
                } else {
                    false
                }
            })
        });
        if is_ok {
            oks.insert(v);
        }
    }
    let len = oks.len();
    if len == 0 {
        println!("0");
    } else {
        let oks = oks.iter().sorted().map(|ok| ok.to_string()).join("\n");
        println!("{}\n{}", len, oks);
    }
}
