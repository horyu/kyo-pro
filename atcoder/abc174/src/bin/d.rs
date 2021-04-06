#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        cc: Chars
    };
    let mut rs = 0;
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        if cc[l] == 'W' {
            while (l < r) && (cc[r] == 'W') {
                r -= 1;
            }
            if l == r {
                break;
            }
            r -= 1;
            rs += 1;
        }
        l += 1;
    }
    println!("{}", rs);
}
