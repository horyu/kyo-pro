#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        x1: isize,
        y1: isize,
        x2: isize,
        y2: isize,
    };
    let get_kouho = |x: isize, y: isize| {
        vec![
            (x + 1, y + 2),
            (x + 2, y + 1),
            (x + 2, y - 1),
            (x + 1, y - 2),
            (x - 1, y - 2),
            (x - 2, y - 1),
            (x - 2, y + 1),
            (x - 1, y + 2),
        ]
    };
    let mut hs = HashSet::new();
    for (x, y) in get_kouho(x1, y1) {
        hs.insert((x, y));
    }
    for (x, y) in get_kouho(x2, y2) {
        if hs.contains(&(x, y)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
