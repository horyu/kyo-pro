#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    const N: usize = 1001;
    let mut va = [false; N];
    let mut vb = [false; N];
    for a in aa {
        va[a] = true;
    }
    for b in bb {
        vb[b] = true;
    }
    println!("{}", (1..N).filter(|&i| va[i] != vb[i]).join(" "));
}
