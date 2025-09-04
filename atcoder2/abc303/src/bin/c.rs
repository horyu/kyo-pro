#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        n: usize,
        m: usize,
        mut h: usize,
        k: usize,
        s: Chars,
        xxyy: [(isize, isize); m],
    };
    let mut hs: HashSet<(isize, isize)> = xxyy.iter().copied().collect();
    let mut x = 0;
    let mut y = 0;
    for c in s {
        if h == 0 {
            println!("No");
            return;
        }
        h -= 1;
        match c {
            'R' => {
                x += 1;
            }
            'L' => {
                x -= 1;
            }
            'U' => {
                y += 1;
            }
            'D' => {
                y -= 1;
            }
            _ => unreachable!(),
        }
        if h < k && hs.remove(&(x, y)) {
            h = k;
        }
    }
    println!("Yes");
}
