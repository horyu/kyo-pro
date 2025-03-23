#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::{Additive, Segtree};
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
        pp: [Usize1; n],
    };
    // https://atcoder.jp/contests/abc392/editorial/12145
    // https://atcoder.jp/contests/abc392/submissions/63877896
    let mut vv = vec![0; n];
    let mut st = Segtree::<Additive<usize>>::from(vec![1; n]);
    for (i, p) in pp.into_iter().enumerate().rev() {
        let pos = st.max_right(0, |&x| x <= p);
        vv[pos] = i + 1;
        st.set(pos, 0);
    }
    let rs = vv.iter().join(" ");
    println!("{rs}");
}
