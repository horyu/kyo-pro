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
        t: usize,
    };
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            cc: [usize; n],
            uuvv: [(Usize1, Usize1); m],
        };
        let mut g = vec![vec![]; n];
        for (u, v) in uuvv {
            g[u].push(v);
            g[v].push(u);
        }
        let mut qq = VecDeque::new();
        let mut pushed = HashSet::new();
        qq.push_back((0, n - 1, 0));
        pushed.insert((0, n - 1));

        let mut rs = None;
        'outer: while let Some((qi, qj, qd)) = qq.pop_front() {
            for &i in &g[qi] {
                for &j in &g[qj] {
                    if cc[i] != cc[j] && pushed.insert((i, j)) {
                        if i == n - 1 && j == 0 {
                            rs = Some(qd + 1);
                            break 'outer;
                        }
                        qq.push_back((i, j, qd + 1));
                    }
                }
            }
        }

        let rs = rs.unwrap_or(-1);
        println!("{rs}");
    }
}
