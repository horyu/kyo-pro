#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::Dsu;
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
    };
    let mut cc = vec![0usize; n];
    let mut uf = UnionFind::new(n);
    for (u, v) in uuvv {
        cc[u] += 1;
        cc[v] += 1;
        if !uf.union(u, v) {
            println!("No");
            return;
        }
    }
    let hm = cc.into_iter().counts();
    if n == m + 1 && hm.get(&1) == Some(&2) && hm.get(&2).copied().unwrap_or_default() == n - 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
