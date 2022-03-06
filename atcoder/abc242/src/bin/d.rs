#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        s: Chars,
        q: usize,
        ttkk: [(usize, Usize1); q]
    };
    for (t, k) in ttkk {
        println!("{}", rec(&s, t, k, &mut Vec::new()));
    }
}

fn rec(s: &[char], t: usize, k: usize, llrr: &mut Vec<usize>) -> char {
    if t > 0 && k > 0 {
        llrr.push(k % 2);
        return rec(s, t - 1, k / 2, llrr);
    }
    let mut c = if t == 0 {
        s[k]
    } else {
        ['A', 'B', 'C'][(s[0] as usize - ('A' as usize) + t) % 3]
    };
    while let Some(lr) = llrr.pop() {
        let arr = match c {
            'A' => ['B', 'C'],
            'B' => ['C', 'A'],
            'C' => ['A', 'B'],
            _ => unreachable!(),
        };
        c = arr[lr];
    }
    c
}
