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
        n: usize,
        aaa: [[usize; n]; n],
        m: usize,
        xxyy: [(Usize1, Usize1); m]
    };
    let mut rs = usize::MAX;
    let mut ngs = vec![vec![false; n]; n];
    for (x, y) in xxyy {
        ngs[x][y] = true;
        ngs[y][x] = true;
    }
    for ii in (0..n).permutations(n) {
        if ii
            .iter()
            .copied()
            .tuple_windows()
            .any(|(ai, aj)| ngs[ai][aj])
        {
            continue;
        }
        let mut tmp = 0;
        for (j, ai) in ii.iter().copied().enumerate() {
            tmp += aaa[ai][j];
        }
        rs = rs.min(tmp);
    }
    if rs == usize::MAX {
        println!("-1");
    } else {
        println!("{rs}");
    }
}
