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
        q: usize,
        xx: [Usize1; q],
    };
    let mut hs = HashSet::new();
    // cc[i] = クエリiを処理した後の hs の要素数
    let mut cc = vec![0; q];
    // vvv[i] = 数値 i が最後に追加されたクエリの番号
    let mut vvv = vec![vec![]; n];

    for (i, x) in xx.iter().copied().enumerate() {
        if hs.insert(x) {
            cc[i] = hs.len();
        } else {
            hs.remove(&x);
            cc[i] = hs.len();
        }

        if x < n {
            vvv[x].push(i);
        }
    }
    for i in hs {
        debug_assert!(vvv[i].len() & 1 == 1);
        vvv[i].push(q);
    }
    // ss = ccの累積和
    // eprintln!("cc: {cc:?}");
    let ss = chain([0], cc).cumsum::<usize>().collect_vec();
    // eprintln!("ss: {ss:?}");

    let ww = vvv
        .into_iter()
        .map(|vv| {
            let mut sum = 0;
            // eprintln!("{vv:?}");
            for (l, r) in vv.into_iter().tuples() {
                sum += ss[r] - ss[l];
            }
            sum
        })
        .collect_vec();
    let rs = ww.iter().join(" ");
    println!("{rs}");
}
