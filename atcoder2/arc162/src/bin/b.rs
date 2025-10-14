#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        n: usize,
        mut i2p: [Usize1; n],
    };
    // 4321 -> 4[21]3 -> 1[42]3 -> 1[23]4
    // 4312 -> 1[43]2 -> 12[43]
    // 35412 -> 3[41]52 -> 1[34]52 -> 14[52]3 -> 12[45]3 -> 123[45]
    // 35421 -> 3[21]54 -> 1[32]54 -> 1[25]34 -> 12345
    // 53412 -> 5[41]32 -> 1[54]32 -> 15[32]4 -> 12[53]4 -> 12[34]5
    // 54321 -> 5[21]43 -> 1[52]43 -> 1[24]53 -> 123[45]
    // 54312 -> 5[31]42 -> 1[53]42 -> 15[42]3 -> 12[54]3 -> 123[54]
    //       -> [12]543
    let mut p2i = vec![0; n];
    for (i, p) in i2p.iter().copied().enumerate() {
        p2i[p] = i;
    }
    let mut rrss = vec![];
    for p in 0..(n - 2) {
        if p2i[p] == p {
            // 正しい位置
        } else if p2i[p] == p + 1 {
            // x[py] -> [py]x と移動させる
            i2p[p..=(p + 2)].rotate_left(1);
            rrss.push(format!("{} {}", p + 2, p));
            p2i[i2p[p]] = p;
            p2i[i2p[p + 1]] = p + 1;
            p2i[i2p[p + 2]] = p + 2;
        } else {
            if p2i[p] == n - 1 {
                // ...[xy]p -> ...p[xy] と移動させる
                i2p[(n - 3)..n].rotate_right(1);
                rrss.push(format!("{} {}", n - 2, n - 2));
                for k in (n - 3)..n {
                    p2i[i2p[k]] = k;
                }
                eprintln!("@{i2p:?}");
            }
            // どこかの px を p の位置に持ってくる
            let j = p2i[p] + 1;
            i2p[p..=j].rotate_right(2);
            for k in p..=j {
                p2i[i2p[k]] = k;
            }
            rrss.push(format!("{} {}", j, p));
        }
        eprintln!("{i2p:?}");
    }
    eprintln!("{i2p:?}");
    if i2p.into_iter().enumerate().all(|(i, p)| i == p) {
        println!("Yes");
        println!("{}", rrss.len());
        for rs in rrss {
            println!("{rs}");
        }
    } else {
        println!("No");
        for rs in rrss {
            eprintln!("{rs}");
        }
    }
}
