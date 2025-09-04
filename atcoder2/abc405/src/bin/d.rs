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
        mut ss: [Chars; h],
    };
    let mut qq = VecDeque::new();
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            if c == 'E' {
                qq.push_back((i, j));
            }
        }
    }
    while let Some((i, j)) = qq.pop_front() {
        if 0 < i && ss[i - 1][j] == '.' {
            ss[i - 1][j] = 'v';
            qq.push_back((i - 1, j));
        }
        if i < h - 1 && ss[i + 1][j] == '.' {
            ss[i + 1][j] = '^';
            qq.push_back((i + 1, j));
        }
        if 0 < j && ss[i][j - 1] == '.' {
            ss[i][j - 1] = '>';
            qq.push_back((i, j - 1));
        }
        if j < w - 1 && ss[i][j + 1] == '.' {
            ss[i][j + 1] = '<';
            qq.push_back((i, j + 1));
        }
    }
    for s in ss {
        println!("{}", s.iter().join(""));
    }
}
