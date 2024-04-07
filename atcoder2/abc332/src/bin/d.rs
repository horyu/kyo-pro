#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(map_try_insert)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        h: usize,
        w: usize,
        aaa: [[usize; w]; h],
        bbb: [[usize; w]; h],
    };
    // https://atcoder.jp/contests/abc332/editorial/7922
    let mut hm = HashMap::new();
    hm.insert(aaa.clone(), 0);
    let mut qq = VecDeque::new();
    qq.push_back(aaa);
    while let Some(aaa) = qq.pop_front() {
        let cnt = hm.get(&aaa).copied().unwrap();
        if aaa == bbb {
            println!("{cnt}");
            return;
        }

        for i in 0..(h - 1) {
            let mut ccc = aaa.clone();
            ccc.swap(i, i + 1);
            if !hm.contains_key(&ccc) {
                hm.insert(ccc.clone(), cnt + 1);
                qq.push_back(ccc);
            }
        }
        for j in 0..(w - 1) {
            let mut ccc = aaa.clone();
            for i in 0..h {
                ccc[i].swap(j, j + 1);
            }
            if !hm.contains_key(&ccc) {
                hm.insert(ccc.clone(), cnt + 1);
                qq.push_back(ccc);
            }
        }
    }

    println!("-1");
}
