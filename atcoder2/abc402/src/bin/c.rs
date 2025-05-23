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
    };
    let mut a2i = vec![vec![]; n];
    let mut kk = vec![0; m];
    for i in 0..m {
        input! {k: usize, aa: [Usize1; k]};
        kk[i] = k;
        for a in aa {
            a2i[a].push(i);
        }
    }
    input! {bb: [Usize1; n]};
    let mut rs = 0;
    for b in bb {
        for i in a2i[b].iter().copied() {
            kk[i] -= 1;
            if kk[i] == 0 {
                rs += 1;
            }
        }
        println!("{rs}");
    }
}
