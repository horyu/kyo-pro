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
        m: usize,
        aa: [usize; n],
    };
    const MAX: usize = 2e5 as usize;
    let a2ii = aa
        .iter()
        .copied()
        .enumerate()
        .map(|(i, a)| (a, i))
        .into_group_map();

    // 転倒数を求める
    let mut rs = 0;
    let mut ft = ac_library::FenwickTree::new(MAX, 0usize);
    for (i, a) in aa.iter().copied().enumerate() {
        rs += i - ft.sum(..=a);
        ft.add(a, 1);
    }
    println!("{rs}");
    for k in 1..m {
        if let Some(ii) = a2ii.get(&(m - k)) {
            // iiのアイテムが全て0になる
            // k=1 => rs = rs + (0..i)内の(0..m-1)の数 - (i..)内の(0..m-1)の数
            // k=2 => rs = rs + (0..i)内の(0..m-2)&(m-1..m)の数 - (i..)内の(0..m-2)&(m-1..m)の数
            let len = ii.len();
            for (cnt, i) in ii.iter().copied().enumerate() {
                rs += i - cnt;
                rs -= n - 1 - i - (len - 1 - cnt);
            }
        }
        println!("{rs}");
    }
}
