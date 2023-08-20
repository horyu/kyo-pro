#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        xy: [usize; 2],
        abc: [usize; 3],
    };
    // ABC | AB
    // ABC | AC
    for abc in abc.into_iter().permutations(3) {
        let (a, b, c) = (abc[0], abc[1], abc[2]);
        for xy in xy.iter().copied().permutations(2) {
            let (x, y) = (xy[0], xy[1]);
            let yy = y.saturating_sub(a.div_ceil(x));
            if yy == 0 {
                continue;
            }
            for pq in [x, yy].into_iter().permutations(2) {
                let (p, q) = (pq[0], pq[1]);
                if b.div_ceil(p) + c.div_ceil(p) <= q {
                    // dbg!([x, y], [p, q], [a, b, c]);
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
