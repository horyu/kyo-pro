#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        s: Chars
    };
    let mut t = std::collections::VecDeque::new();
    let mut rev = false;
    for c in s {
        if c == 'R' {
            rev = !rev;
            continue;
        }
        if rev {
            match t.get(0) {
                Some(&tc) if tc == c => {
                    t.pop_front();
                }
                _ => {
                    t.push_front(c);
                }
            }
        } else {
            match t.iter().last() {
                Some(&tc) if tc == c => {
                    t.pop_back();
                }
                _ => {
                    t.push_back(c);
                }
            }
        }
    }
    let s: String = if rev {
        t.iter().rev().collect()
    } else {
        t.iter().collect()
    };
    println!("{}", s);
}
