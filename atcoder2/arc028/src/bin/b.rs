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
        // xx: [usize; n],
        i2x: [Usize1; n],
    };
    // 順位が i 位の参加者の年齢が全参加者のうち Xi​ 番目に若い
    let mut bh = BinaryHeap::new();
    for (i, x) in i2x.into_iter().enumerate() {
        bh.push((x, i));
        if k < bh.len() {
            bh.pop();
        }
        // eprintln!("{i} {x} {bh:?}");
        if k - 1 <= i {
            // 「i+K−1 位以上の人のうち、K 番目に若い人」の順位
            let rs = bh.peek().unwrap().1 + 1;
            println!("{rs}");
        }
    }
}
