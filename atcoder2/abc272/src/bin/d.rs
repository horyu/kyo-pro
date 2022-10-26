#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut dd = vec![];
    for i in 0..=m.sqrt() {
        let ii = i.pow(2);
        let j = (m - ii).sqrt();
        let jj = j.pow(2);
        if ii + jj == m {
            dd.push((i, j));
        }
    }
    let mut qq = VecDeque::new();
    let mut lll = vec![vec![-1; n]; n];
    qq.push_back((0usize, 0usize));
    lll[0][0] = 0;
    while let Some((qi, qj)) = qq.pop_front() {
        for &(dx, dy) in &dd {
            let mut ii = vec![];
            let mut jj = vec![];
            if qi + dx < n {
                ii.push(qi + dx);
            }
            if dx <= qi {
                ii.push(qi - dx);
            }
            if qj + dy < n {
                jj.push(qj + dy);
            }
            if dy <= qj {
                jj.push(qj - dy);
            }
            for (i, j) in ii.into_iter().cartesian_product(jj.into_iter()) {
                if lll[i][j] == -1 {
                    lll[i][j] = lll[qi][qj] + 1;
                    qq.push_back((i, j));
                }
            }
        }
    }
    for ll in lll {
        println!("{}", ll.iter().join(" "));
    }
}
