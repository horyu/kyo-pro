#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        _n: usize,
        a: Chars,
        b: Chars,
        c: Chars,
    };
    let mut cnt = 0;
    for ((a, b), c) in a.into_iter().zip(b.into_iter()).zip(c.into_iter()) {
        let mut sames = 0;
        sames += (a == b) as i32;
        sames += (b == c) as i32;
        sames += (c == a) as i32;
        cnt += if sames >= 2 {
            0
        } else if sames == 1 {
            1
        } else {
            2
        };
    }
    println!("{}", cnt);
}
