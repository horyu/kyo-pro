#![allow(clippy::manual_saturating_arithmetic, clippy::needless_range_loop)]
#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    };
    let mut vv = vec![vec![0; w]; h];
    for (y, s) in ss.iter().enumerate() {
        for (x, &c) in s.iter().enumerate() {
            if c == '#' {
                use std::cmp::min;
                for xx in x.checked_sub(1).unwrap_or(0)..=min(x + 1, w - 1) {
                    for yy in y.checked_sub(1).unwrap_or(0)..=min(y + 1, h - 1) {
                        vv[yy][xx] += 1;
                    }
                }
            }
        }
    }
    let mut s = String::new();
    for y in 0..h {
        for x in 0..w {
            let c = if ss[y][x] == '#' {
                '#'
            } else {
                std::char::from_digit(vv[y][x], 10).unwrap()
            };
            s.push(c);
        }
        s.push('\n');
    }
    print!("{}", s);
}
