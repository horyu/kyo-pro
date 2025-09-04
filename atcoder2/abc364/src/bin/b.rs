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
        mut i: usize,
        mut j: usize,
        mut ccc: [Chars; h],
        x: Chars,
    };
    for cc in ccc.iter_mut() {
        cc.insert(0, '#');
        cc.push('#');
    }
    ccc.insert(0, vec!['#'; w + 2]);
    for c in x {
        match c {
            'L' => {
                if 1 < j && ccc[i][j - 1] != '#' {
                    j -= 1;
                }
            }
            'R' => {
                if j < w && ccc[i][j + 1] != '#' {
                    j += 1;
                }
            }
            'U' => {
                if 1 < i && ccc[i - 1][j] != '#' {
                    i -= 1;
                }
            }
            'D' => {
                if i < h && ccc[i + 1][j] != '#' {
                    i += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{i} {j}");
}
