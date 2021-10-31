#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut mae = vec![std::usize::MAX; n + 1];
    let mut ato = vec![std::usize::MAX; n + 1];
    for _ in 0..q {
        input! {c: usize, x: usize};
        if c == 3 {
            let mut v = VecDeque::new();
            let mut pre = x;
            while pre != std::usize::MAX {
                v.push_front(pre);
                pre = mae[pre]
            }
            let mut next = ato[x];
            while next != std::usize::MAX {
                v.push_back(next);
                next = ato[next];
            }
            println!("{} {}", v.len(), v.iter().join(" "));
            continue;
        }
        input! {y: usize};
        if c == 1 {
            mae[y] = x;
            ato[x] = y;
        } else {
            // c == 2
            mae[y] = std::usize::MAX;
            ato[x] = std::usize::MAX;
        }
    }
}
