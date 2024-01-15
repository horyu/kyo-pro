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
        aa: [usize; n],
        bb: [usize; n],
    };
    let aa = aa.into_iter().enumerate().map(|(i, a)| a + i).collect_vec();
    let bb = bb.into_iter().enumerate().map(|(i, b)| b + i).collect_vec();
    if aa.iter().copied().counts() != bb.iter().copied().counts() {
        println!("-1");
        return;
    }
    // eprintln!("{aa:?}");
    // eprintln!("{bb:?}");
    // aaをbbにするソート回数 == 転倒数 を求める
    let mut mm = multimap::MultiMap::new();
    for (i, &a) in aa.iter().enumerate() {
        mm.insert(a, i);
    }
    // eprintln!("{}", mm.iter().map(|(k, v)| format!("({k}:{v:?})")).join(" "));
    let mut cc = vec![0; n];
    for (i, b) in bb.into_iter().enumerate().rev() {
        cc[i] = mm.get_vec_mut(&b).unwrap().pop().unwrap();
    }
    // eprintln!("{cc:?}");
    // ccの転倒数を求める
    let mut bit = ac_library::FenwickTree::new(n, 0usize);
    let mut rs = 0;
    for (i, c) in cc.iter().copied().enumerate() {
        rs += i - bit.sum(..c);
        bit.add(c, 1);
    }
    println!("{rs}");
}
