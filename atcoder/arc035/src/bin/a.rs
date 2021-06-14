#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    let mut l = 0usize;
    let mut r = s.len() - 1;
    while l < r {
        match (s[l], s[r]) {
            ('*', _) | (_, '*') => (),
            (a, b) if a == b => (),
            _ => {
                println!("NO");
                return;
            }
        }
        l += 1;
        r -= 1;
    }
    println!("YES");
}
