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
        aa: [usize; n],
    };
    let pf = PrimeFact::new(2e5 as usize);
    let mut zeros = 0;
    let mut counter = counter::Counter::<_>::new();
    let mut rs = 0;
    for (i, a) in aa.iter().copied().enumerate() {
        if a == 0 {
            rs += i;
            zeros += 1;
            continue;
        }
        let mut bts = BTreeSet::new();
        for (k, v) in pf.get(a) {
            if v % 2 == 1 {
                bts.insert(k);
            }
        }
        rs += counter[&bts] + zeros;
        counter[&bts] += 1;
    }
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
        while 1 < n {
            *hm.entry(self.spf[n]).or_insert(0) += 1usize;
            n /= self.spf[n];
        }
        hm
    }
}
