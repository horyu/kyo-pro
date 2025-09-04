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
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut ddd = vec![vec![!0; n]; n];
    for (a, b, c) in aabbcc {
        ddd[a][b] = c;
        ddd[b][a] = c;
    }
    let mut rs = 0;
    for ii in (0..n).permutations(n) {
        let mut tmp = 0;
        for (il, ir) in ii.iter().copied().tuple_windows() {
            if ddd[il][ir] != !0 {
                tmp += ddd[il][ir];
            } else {
                break;
            }
        }
        rs = rs.max(tmp);
    }
    println!("{rs}");
}
