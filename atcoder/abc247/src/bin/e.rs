#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
#![feature(let_chains)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        aa: [usize; n]
    };
    let bbb = aa
        .into_iter()
        .group_by(|&a| (y..=x).contains(&a))
        .into_iter()
        .filter_map(|(k, g)| {
            if k {
                Some(g.into_iter().collect_vec())
            } else {
                None
            }
        })
        .collect_vec();
    let mut rs = 0;
    for bb in bbb {
        let len = bb.len();
        // 尺取方
        let (mut i, mut j, mut cx, mut cy) = (0, 0, 0, 0);
        while i < len {
            while j < len && (cx == 0 || cy == 0) {
                if bb[j] == x {
                    cx += 1;
                }
                if bb[j] == y {
                    cy += 1;
                }
                j += 1;
            }
            if cx > 0 && cy > 0 {
                rs += len + 1 - j;
            }
            if bb[i] == x {
                cx -= 1;
            }
            if bb[i] == y {
                cy -= 1;
            }
            i += 1;
        }
        // 二部探索
        // if y == x {
        //     rs += len * (len + 1) / 2;
        //     continue;
        // }
        // let mut yy = BTreeSet::new();
        // let mut xx = BTreeSet::new();
        // for (i, b) in bb.into_iter().enumerate() {
        //     if b == y {
        //         yy.insert(i);
        //     } else if b == x {
        //         xx.insert(i);
        //     }
        // }
        // for i in 0..len {
        //     if let Some(yi) = yy.range(i..).next() && let Some(xi) = xx.range(i..).next() {
        //         let max = yi.max(xi);
        //         rs += len - max;
        //     } else {
        //         break;
        //     }
        // }
    }
    println!("{rs}");
}
