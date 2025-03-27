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
        aa: [usize; n],
    };
    let pf = PrimeFact::new(1_000_000);

    let mut cc = vec![0; 1000001];
    let mut ddd = Vec::with_capacity(n);
    for (i, a) in aa.iter().copied().enumerate() {
        let hm = pf.get(a);
        let mut dd = vec![1];
        for (x, y) in hm {
            let mut new_dd = dd.clone();
            let mut xx = x;
            for _ in 0..y {
                for d in dd.iter().copied() {
                    new_dd.push(d * xx);
                }
                xx *= x;
            }
            dd = new_dd;
        }
        for x in dd.iter().copied() {
            cc[x] += 1;
        }
        ddd.push(dd);
    }
    panic!("試算ミス？");
    // for dd in ddd {
    //     let mut rs = 0;
    //     for d in dd {
    //         if k <= cc[d] {
    //             rs = rs.max(d);
    //         }
    //     }
    //     println!("{rs}");
    // }
}

// https://algo-logic.info/prime-fact/
struct PrimeFact {
    spf: Vec<usize>,
}
impl PrimeFact {
    fn new(n: usize) -> Self {
        let mut spf = (0..=n).collect_vec();
        for i in (2..=n).take_while(|x| x * x <= n) {
            if spf[i] == i {
                for j in (i..=n).step_by(i) {
                    if spf[j] == j {
                        spf[j] = i;
                    }
                }
            }
        }
        Self { spf }
    }

    fn get(&self, mut n: usize) -> HashMap<usize, usize> {
        let mut hm = HashMap::new();
        while n != 1 {
            *hm.entry(self.spf[n]).or_insert(0) += 1usize;
            n /= self.spf[n];
        }
        hm
    }
}
