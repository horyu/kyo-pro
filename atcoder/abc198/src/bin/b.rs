#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        mut n: Chars,
    };
    let len = n.len();
    if len == 1 {
        println!("Yes");
        return;
    }
    loop {
        if let Some(&last) = n.last() {
            if last == '0' {
                n.pop();
            } else {
                break;
            }
        }
    }
    let s: String = n.iter().collect();
    let rs: String = n.iter().rev().collect();
    println!("{}", ["No", "Yes"][(s == rs) as usize]);
}
