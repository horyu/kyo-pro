#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        h: usize,
        w: usize,
        ss: [Chars; h]
    };
    let mut rs = 1;
    // 斜めに見ていき、全部白なら*2 、赤青混合なら*0
    for r in 0..=(h + w - 2) {
        let (mut red, mut blue) = (0, 0);
        for i in 0..=r.min(h - 1) {
            let j = r - i;
            if j >= w {
                continue;
            }
            // eprintln!("{r}: {i} {j}");
            match ss[i][j] {
                'R' => red += 1,
                'B' => blue += 1,
                _ => (),
            };
        }
        if red > 0 && blue > 0 {
            rs = 0;
        } else if red == 0 && blue == 0 {
            rs = rs * 2 % 998244353;
        }
    }
    println!("{rs}");
}
