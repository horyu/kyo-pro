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
        n: usize,
        m: usize,
        aabbcc: [(Usize1, Usize1, usize); m],
    };
    let mut rs = vec![!0usize; n];
    let mut g = vec![HashMap::new(); n];
    for (a, b, c) in aabbcc.iter().copied() {
        if a == b {
            rs[a] = rs[a].min(c);
        } else {
            let e = g[a].entry(b).or_insert(!0);
            *e = c.min(*e);
        }
    }
    for i in 0..n {
        use std::cmp::Reverse;
        let mut bh = BinaryHeap::from_iter(g[i].iter().map(|(&k, &v)| (Reverse(v), k)));
        let mut confirmed = vec![false; n];
        while let Some((Reverse(d), j)) = bh.pop() {
            if j == i {
                rs[i] = rs[i].min(d);
                break;
            }
            if !confirmed[j] {
                confirmed[j] = true;
                for (&k, &c) in g[j].iter() {
                    if !confirmed[k] {
                        bh.push((Reverse(d + c), k));
                    }
                }
            }
        }
    }
    for rs in rs {
        if rs == !0 {
            println!("-1");
        } else {
            println!("{rs}");
        }
    }
}
