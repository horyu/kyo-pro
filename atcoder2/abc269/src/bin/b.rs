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
        ss: [Chars; 10],
    };
    let mut x = (0, 0);
    let mut y = (0, 0);
    'outer: for i in 0..10 {
        for j in 0..10 {
            if ss[i][j] == '#' {
                x = (i, j);
                break 'outer;
            }
        }
    }
    'outer: for i in (0..10).rev() {
        for j in (0..10).rev() {
            if ss[i][j] == '#' {
                y = (i, j);
                break 'outer;
            }
        }
    }
    println!("{} {}", x.0 + 1, y.0 + 1);
    println!("{} {}", x.1 + 1, y.1 + 1);
}
