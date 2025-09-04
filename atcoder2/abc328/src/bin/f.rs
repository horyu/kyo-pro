#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
        q: usize,
        aabbdd: [(Usize1, Usize1, isize); q],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut xx = vec![0isize; n];
    let mut hhss = (0..n)
        .map(|i| HashSet::<usize>::from_iter([i]))
        .collect_vec();
    let mut rrss = vec![];
    for (qi, (a, b, d)) in aabbdd.iter().copied().enumerate() {
        let qi = qi + 1;
        if a == b {
            if d == 0 {
                rrss.push(qi);
            }
            continue;
        }
        if dsu.same(a, b) {
            if xx[a] - xx[b] == d {
                rrss.push(qi);
            }
            continue;
        }

        // xx[a] - xx[b] = d となるように小さいサイズのHashSetに含まれるxxを更新する
        let al = dsu.leader(a);
        let bl = dsu.leader(b);
        let asize = dsu.size(a);
        let bsize = dsu.size(b);

        let newl = dsu.merge(a, b);
        let diff = d - xx[a] + xx[b];
        if asize < bsize {
            let tmp = std::mem::take(&mut hhss[al]);
            for i in tmp.iter().copied() {
                xx[i] += diff;
            }
            hhss[bl].extend(tmp);
        } else {
            let tmp = std::mem::take(&mut hhss[bl]);
            for i in tmp.iter().copied() {
                xx[i] -= diff;
            }
            hhss[al].extend(tmp);
        }
        if hhss[newl].is_empty() {
            hhss.swap(al, bl);
        }

        rrss.push(qi);
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
