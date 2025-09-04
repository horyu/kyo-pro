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
        k: usize,
        mut aa: [usize; n],
        mut bb: [usize; n],
        mut cc: [usize; n],
    };
    aa.sort_unstable();
    aa.reverse();
    bb.sort_unstable();
    bb.reverse();
    cc.sort_unstable();
    cc.reverse();
    let f = |[a, b, c]: [usize; 3]| {
        let [a, b, c] = [aa[a], bb[b], cc[c]];
        a * b + b * c + c * a
    };

    let mut bh = BinaryHeap::new();
    let mut pushed = HashSet::new();
    bh.push((f([0; 3]), [0; 3]));
    pushed.insert([0; 3]);
    let mut cnt = 1;
    while let Some((v, arr)) = bh.pop() {
        if cnt == k {
            println!("{v}");
            return;
        }
        cnt += 1;
        for pos in 0..3 {
            let mut arr = arr;
            arr[pos] += 1;
            if arr.iter().all(|&x| x < n) && pushed.insert(arr) {
                bh.push((f(arr), arr));
            }
        }
    }
}
