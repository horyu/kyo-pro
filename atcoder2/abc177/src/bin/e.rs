#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use counter::Counter;
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
        aa: [usize; n],
    };
    let pf = PrimeFact::new(1e6 as usize);
    let mut counter = Counter::<usize>::new();
    for a in aa.iter().copied() {
        for x in pf.get(a).into_keys() {
            counter[&x] += 1;
        }
    }
    let max = counter.values().copied().max().unwrap_or_default();
    let rs = if n == max {
        "not coprime"
    } else if 1 < max {
        "setwise coprime"
    } else {
        "pairwise coprime"
    };
    println!("{rs}");
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
