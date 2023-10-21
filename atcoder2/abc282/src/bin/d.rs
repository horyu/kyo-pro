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
        uuvv: [(Usize1, Usize1); m],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut rs = 0;
    let mut sum = 0usize;
    let mut cc = vec![!0; n];
    for i in 0..n {
        if cc[i] != !0 {
            continue;
        }
        let mut qq = VecDeque::new();
        qq.push_back(i);
        let mut ii = vec![i];
        cc[i] = 0;
        // c0 -> c1 への接続だけカウントする
        let mut c1 = 0;
        while let Some(qi) = qq.pop_front() {
            for &qj in &g[qi] {
                if cc[qj] != !0 {
                    if cc[qi] == cc[qj] {
                        println!("0");
                        return;
                    }
                    continue;
                }
                cc[qj] = 1 - cc[qi];
                if cc[qj] == 1 {
                    c1 += 1;
                }
                qq.push_back(qj);
                ii.push(qj);
            }
        }

        rs += sum * ii.len();
        for &i in &ii {
            if cc[i] == 0 {
                rs += c1 - g[i].len();
            }
        }
        sum += ii.len();
    }
    println!("{rs}");
}
