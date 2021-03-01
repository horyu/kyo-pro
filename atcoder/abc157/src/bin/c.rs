#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        scsc: [(Usize1, usize); m]
    };
    let mut v = vec![None; n];
    for (s, c) in scsc {
        if let Some(vc) = v[s] {
            if vc != c {
                println!("-1");
                return;
            }
        } else {
            v[s] = Some(c);
        }
    }
    if n > 1 {
        if let Some(0) = v[0] {
            println!("-1");
            return;
        } else if v[0].is_none() {
            v[0] = Some(1);
        }
    }
    let rs = v
        .into_iter()
        .map(|opt| opt.unwrap_or_default().to_string())
        .join("");
    println!("{}", rs);
}
