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
        d: usize,
        aa: [usize; n],
    };
    if d == 0 {
        let rs = aa
            .into_iter()
            .counts()
            .into_values()
            .map(|c| c - 1)
            .sum::<usize>();
        println!("{rs}");
        return;
    }
    let mut dsu = ac_library::Dsu::new(n);
    let mut hm = HashMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        for k in [a.wrapping_sub(d), a, a + d] {
            if let Some(&j) = hm.get(&k) {
                dsu.merge(i, j);
            }
        }
        hm.insert(a, i);
    }
    let mut rs = 0;
    for ii in dsu.groups() {
        // (d=1) 1 1 2 3 4 4 のときWA
        let bb = ii.iter().copied().map(|i| aa[i]).collect_vec();
        let min = bb.iter().copied().min().unwrap();
        // [min + 2n*d, min + (2n+1)*d]
        let mut cc = [0; 2];
        for b in bb {
            let k = (b - min) / d;
            cc[k % 2] += 1;
        }
        rs += cc[0].min(cc[1]);
    }
    println!("{rs}");
}
