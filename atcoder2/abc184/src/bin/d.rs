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
        a: usize,
        b: usize,
        c: usize,
        // abc: [usize; 3]
    };
    let mut abc = [a, b, c];
    let ori_sum = abc.iter().sum::<usize>();
    abc.sort_unstable();
    let mut hm = HashMap::new();
    hm.insert(abc, 1.0);
    let mut rs = 0.0;
    while !hm.is_empty() {
        let mut new_hm = HashMap::new();
        for (abc, c) in hm {
            for i in 0..3 {
                let mut xyz = abc;
                let sum = xyz.iter().sum::<usize>();
                let p = c * xyz[i] as f64 / sum as f64;
                if xyz[i] == 99 {
                    rs += p * (1 + sum - ori_sum) as f64;
                    // eprintln!("{xyz:?} {}", p);
                } else {
                    xyz[i] += 1;
                    xyz.sort_unstable();
                    *new_hm.entry(xyz).or_insert(0.0) += p;
                }
            }
        }
        hm = new_hm;
    }
    println!("{rs}");
}
