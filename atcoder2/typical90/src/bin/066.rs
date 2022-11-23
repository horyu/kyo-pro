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
        llrr: [(Usize1, Usize1); n],
    };
    let mut rs = 0.0;
    let mut arr = [0.0; 100];
    for (l, r) in llrr {
        let mut new_arr = arr;
        let size = (r - l + 1) as f64;
        for i in l..=r {
            let bigs = arr[(i + 1)..].iter().sum::<f64>();
            if 0.0 < bigs {
                rs += bigs / size;
            }
            new_arr[i] += 1.0 / size;
        }
        arr = new_arr;
    }
    println!("{rs}");
}
