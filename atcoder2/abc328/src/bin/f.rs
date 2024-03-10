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
        q: usize,
        aabbdd: [(Usize1, Usize1, isize); q],
    };
    let mut dsu = ac_library::Dsu::new(n);
    let mut xx = vec![0isize; n];
    let mut hhss = (0..n)
        .map(|i| HashSet::<usize>::from_iter([i]))
        .collect_vec();
    let mut rrss = vec![];
    for (i, (a, b, d)) in aabbdd.iter().copied().enumerate() {
        let ii = i + 1;
        if a == b {
            if d == 0 {
                rrss.push(ii);
            }
            continue;
        }
        if dsu.same(a, b) {
            if xx[a] - xx[b] == d {
                rrss.push(ii);
            }
            continue;
        }

        // xx[a] - xx[b] = d となるように小さいサイズのHashSetに含まれるxxを更新する
        let la = dsu.leader(a);
        let lb = dsu.leader(b);
        let asize = dsu.size(a);
        let bsize = dsu.size(b);

        let new_l = dsu.merge(a, b);
        let diff = d - xx[a] + xx[b];
        if asize < bsize {
            for &i in &hhss[la] {
                xx[i] += diff;
            }
            let tmp = std::mem::take(&mut hhss[la]);
            hhss[lb].extend(tmp);
            if new_l == la {
                hhss.swap(la, lb);
            }
        } else {
            for &i in &hhss[lb] {
                xx[i] -= diff;
            }
            let tmp = std::mem::take(&mut hhss[lb]);
            hhss[la].extend(tmp);
            if new_l == lb {
                hhss.swap(la, lb);
            }
        }

        rrss.push(ii);
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
