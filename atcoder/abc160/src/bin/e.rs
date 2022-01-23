#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        mut pp: [isize; a],
        mut qq: [isize; b],
        mut rr: [isize; c],
    };
    pp.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    qq.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    rr.sort_unstable_by_key(|&x| std::cmp::Reverse(x));
    // 最初にpp,ppから取る
    let mut vv = vec![];
    vv.extend_from_slice(&pp[..x]);
    vv.extend_from_slice(&qq[..y]);
    vv.sort_unstable();
    for (v, r) in vv.iter_mut().zip(rr.into_iter()) {
        if *v < r {
            *v = r;
        }
    }
    let rs = vv.into_iter().sum::<isize>();
    println!("{rs}");
}
