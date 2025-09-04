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
        h: usize,
        w: usize,
        n: usize,
        t: Chars,
        ss: [Chars; h],
    };
    let mut ng = vec![vec![true; w + 2]; h + 2];
    for (i, s) in ss.into_iter().enumerate() {
        for (j, c) in s.into_iter().enumerate() {
            ng[i + 1][j + 1] = c == '#';
        }
    }
    let mut rs = 0;
    for i in 1..=h {
        'outer: for j in 1..=w {
            if ng[i][j] {
                continue;
            }
            let mut ii = i;
            let mut jj = j;
            for tc in t.iter().copied() {
                match tc {
                    'L' => {
                        jj -= 1;
                    }
                    'R' => {
                        jj += 1;
                    }
                    'U' => {
                        ii -= 1;
                    }
                    'D' => {
                        ii += 1;
                    }
                    _ => unreachable!(),
                }
                if ng[ii][jj] {
                    continue 'outer;
                }
            }
            rs += 1;
        }
    }
    println!("{rs}");
}
