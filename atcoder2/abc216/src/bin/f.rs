#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
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
    // https://atcoder.jp/contests/abc216/editorial/2560
    let ii = (0..n)
        .sorted_unstable_by(|&i, &j| {
            let (ai, aj) = (aa[i], aa[j]);
            let (bi, bj) = (bb[i], bb[j]);
            ai.cmp(&aj).then(bi.cmp(&bj))
        })
        .collect_vec();
    let cc = ii.iter().copied().map(|i| aa[i]).collect_vec();
    let dd = ii.iter().copied().map(|i| bb[i]).collect_vec();

    let mut rs = ModInt998244353::default();
    let mut arr = [ModInt998244353::default(); 5001];
    arr[0] += 1;
    for (c, d) in izip!(cc, dd) {
        let mut new_arr = arr;
        for i in 0..=5000 {
            if d <= i {
                new_arr[i] += arr[i - d];
            }
            if i + d <= c {
                rs += arr[i];
            }
        }
        arr = new_arr;
    }
    println!("{rs}");
}
