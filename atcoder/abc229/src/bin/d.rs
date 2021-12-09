#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars,
        k: usize,
    };
    let mut rs = 0;
    let mut cur = 0;
    let mut l = 0;
    let mut r = 0;
    while r < s.len() {
        if s[r] == '.' {
            cur += 1;
        }
        r += 1;

        while cur > k {
            if s[l] == '.' {
                cur -= 1;
            }
            l += 1;
        }

        rs = rs.max(r - l);
    }
    println!("{}", rs);
}
