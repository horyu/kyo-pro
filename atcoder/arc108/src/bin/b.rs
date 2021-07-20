#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut count = 0;
    let mut v = VecDeque::new();
    for c in s {
        match c {
            'f' => {
                v.push_back(c);
            }
            'o' => {
                if Some(&'f') == v.back() {
                    v.push_back(c);
                } else {
                    v.clear();
                }
            }
            'x' => {
                let len = v.len();
                if len >= 2 && v[len - 2] == 'f' && v[len - 1] == 'o' {
                    count += 1;
                    v.pop_back();
                    v.pop_back();
                } else {
                    v.clear();
                }
            }
            _ => {
                v.clear();
            }
        }
    }
    println!("{}", n - 3 * count);
}
