#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::{
    collections::{HashMap, HashSet},
    mem::swap,
};

fn main() {
    input! {
        mut s: Chars,
        q: usize,
    };
    let mut mae = std::collections::VecDeque::new();
    let mut ato = std::collections::VecDeque::new();
    let mut rev = false;
    for _ in 0..q {
        input! { t: usize };
        if t == 1 {
            rev = !rev;
        } else {
            input! { f: usize, c: char }
            if ((f == 1) && !rev) || ((f != 1) && rev) {
                mae.push_front(c);
            } else {
                ato.push_back(c);
            }
        }
    }
    if rev {
        println!(
            "{}{}{}",
            ato.iter().rev().join(""),
            s.iter().rev().join(""),
            mae.iter().rev().join("")
        );
    } else {
        println!(
            "{}{}{}",
            mae.iter().join(""),
            s.iter().join(""),
            ato.iter().join("")
        );
    }
}
