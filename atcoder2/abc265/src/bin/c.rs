#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        ggg: [Chars; h],
    };
    let mut hs = HashSet::new();
    let mut i = 0;
    let mut j = 0;
    loop {
        if !hs.insert((i, j)) {
            println!("-1");
            return;
        }
        match ggg[i][j] {
            'U' => {
                if i == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                i -= 1;
            }
            'D' => {
                if i == h - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                i += 1;
            }
            'L' => {
                if j == 0 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                j -= 1;
            }
            'R' => {
                if j == w - 1 {
                    println!("{} {}", i + 1, j + 1);
                    return;
                }
                j += 1;
            }
            _ => (),
        }
    }
}
