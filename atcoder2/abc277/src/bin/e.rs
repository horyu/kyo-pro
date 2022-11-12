#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
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
        k: usize,
        uuvvaa: [(Usize1, Usize1, usize); m],
        ss: [Usize1; k],
    };
    let ss = {
        let mut tmp = vec![false; n];
        for s in ss {
            tmp[s] = true
        }
        tmp
    };
    let mut g = vec![vec![vec![]; n]; 2];
    for (u, v, a) in uuvvaa {
        g[a][u].push(v);
        g[a][v].push(u);
    }
    let mut qq = VecDeque::new();
    qq.push_back((0, 0, 1));
    let mut pushed = vec![vec![false; n]; 2];
    while let Some((qi, qc, qa)) = qq.pop_front() {
        let qqaa = if ss[qi] { vec![0, 1] } else { vec![qa] };
        for qa in qqaa {
            for &pi in &g[qa][qi] {
                if pi == n - 1 {
                    println!("{}", qc + 1);
                    return;
                }
                if !pushed[qa][pi] {
                    pushed[qa][pi] = true;
                    qq.push_back((pi, qc + 1, qa));
                }
            }
        }
    }
    println!("-1");
}
