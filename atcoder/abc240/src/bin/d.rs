#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        aa: [usize; n]
    };
    let mut v: Vec<(usize, usize)> = vec![];
    let mut counts = 0;
    let mut pre = std::usize::MAX;
    for a in aa {
        counts += 1;
        if a == pre {
            let e = v.last_mut().unwrap();
            (*e).1 += 1;
            if e.0 == e.1 {
                counts -= e.0;
                v.pop();
            }
            if let Some(e) = v.last() {
                pre = e.0;
            } else {
                pre = std::usize::MAX;
            }
        } else {
            v.push((a, 1));
            pre = a;
        }
        println!("{counts}");
    }
}
