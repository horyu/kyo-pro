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
    let mut hm: HashMap<usize, Vec<usize>> = HashMap::new();
    for iiaa in (1..=n).flat_map(|size| aa.iter().enumerate().combinations(size)) {
        let sum = iiaa.iter().map(|ia| ia.1).sum::<usize>() % 200;
        let ii = iiaa.iter().map(|ia| ia.0 + 1).collect_vec();
        if let Some(jj) = hm.get(&sum) {
            println!("Yes");
            for bb in [jj, &ii] {
                println!("{} {}", bb.len(), bb.iter().join(" "));
            }
            return;
        } else {
            hm.insert(sum, ii);
        }
    }
    println!("No");
}
