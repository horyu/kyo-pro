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
    };
    let mut bts = BTreeSet::new();
    for i in 1.. {
        for x in [2 * 3, 2 * 5, 3 * 5] {
            let y = i * x;
            if y <= 10000 && bts.insert(i * x) && bts.len() == n {
                println!("{}", bts.iter().join(" "));
                // dbg!(bts.iter().copied().tuple_combinations().all(|(a, b)| a.gcd(&b) > 1));
                // dbg!(bts.iter().copied().fold(105, |a, b| a.gcd(&b)));
                return;
            }
        }
    }
}
/*
2,3,5
2.3 2.5 3.5 2.3.5 = 2(3+5+3*5)
2,3,5,7
2.3 2.5 2.7 2.3.5 2.3.7 3.5.7 (2+3+4+5+6+7+8)*(2.3 2.5 2.7 2.3.5 2.3.7)
 */
