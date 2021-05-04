#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    if m == 0 {
        println!("{}", (1..=n).join("\n"));
        return;
    }
    input! {
        dd: [usize; m]
    }
    let mut vv = (0usize..=n).collect_vec();
    for d in dd {
        let pos = vv.iter().position(|&v| v == d).unwrap();
        vv.swap(0, pos);
    }
    println!("{}", vv[1..].iter().join("\n"));
}
