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
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut dd = vec![vec![!0; n]; n];
    for i in 0..n {
        dd[i][i] = 0usize;
    }
    for (a, b, c) in aabbcc {
        dd[a][b] = c;
    }
    let mut rs = 0;
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dd[i][j] = dd[i][j].min(dd[i][k].saturating_add(dd[k][j]));
            }
        }

        for i in 0..n {
            for j in 0..n {
                if dd[i][j] != !0 {
                    rs += dd[i][j];
                }
            }
        }
    }
    println!("{rs}");
}
