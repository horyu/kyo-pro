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
        k: usize,
        aaa: [[usize; n]; n],
        q: usize,
        sstt: [(Usize1, Usize1); q],
    };
    let mut g = vec![vec![]; n];
    for (i, aa) in aaa.iter().enumerate() {
        for (j, a) in aa.iter().copied().enumerate() {
            if 0 < a {
                g[i].push(j);
            }
        }
    }
    // mod n で考える
    'outer: for (s, t) in sstt {
        let ss = s % n;
        let tt = t % n;
        let mut qq = VecDeque::new();
        let mut pushed = vec![false; n];
        qq.push_back((ss, 1));
        pushed[ss] = true;
        while let Some((qi, qd)) = qq.pop_front() {
            for i in g[qi].iter().copied() {
                if tt == i {
                    println!("{qd}");
                    continue 'outer;
                }
                if !pushed[i] {
                    pushed[i] = true;
                    qq.push_back((i, qd + 1));
                }
            }
        }
        println!("-1");
    }
}
