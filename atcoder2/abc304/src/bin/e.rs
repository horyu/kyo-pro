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
        uuvv: [(Usize1, Usize1); m],
        k: usize,
        xxyy: [(Usize1, Usize1); k],
        q: usize,
        ppqq: [(Usize1, Usize1); q],
    };
    let mut uf = UnionFind::new(n);
    for (u, v) in uuvv.iter().copied() {
        uf.union(u, v);
    }
    let ll = (0..n).map(|i| uf.find(i)).collect_vec();
    let mut ngs = HashSet::new();
    for (x, y) in xxyy.iter().copied() {
        let lx = ll[x];
        let ly = ll[y];
        ngs.insert((lx, ly));
        ngs.insert((ly, lx));
    }
    for (p, q) in ppqq.iter().copied() {
        let lp = ll[p];
        let lq = ll[q];
        let rs = ["Yes", "No"][ngs.contains(&(lp, lq)) as usize];
        println!("{rs}");
    }
}
