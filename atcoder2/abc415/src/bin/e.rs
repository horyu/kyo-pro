#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
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
        pp: [usize; h + w - 1],
    };
    let mut hm = HashMap::new();
    hm.insert((0, 0), (0, 0));
    for (k, p) in pp.iter().copied().enumerate() {
        let mut new_hm = HashMap::new();
        for ((i, j), (vp, vn)) in hm {
            let a = aaa[i][j];
            for (ii, jj) in chain!(
                (i + 1 < h).then_some((i + 1, j)),
                (j + 1 < w).then_some((i, j + 1)),
                (i + j == h + w - 2).then_some((i, j))
            ) {
                let e = new_hm.entry((ii, jj)).or_insert((0, usize::MAX));
                let vvp = (vp + a).saturating_sub(p);
                let vvn = vn + p.saturating_sub(vp + a);
                if vvn < e.1 || (vvn == e.1 && e.0 < vvp) {
                    *e = (vvp, vvn);
                    // if (h, w) == (2, 2) {
                    //     eprintln!("[{k}] ({i},{j}) -> ({ii},{jj}): {a}+{vp}-{p}={vvp}, {vn}+{p}-{vp}-{a}={vvn}");
                    // }
                }
            }
        }
        hm = new_hm;
        // if (h, w) == (2, 2) {
        //     for (kk, v) in &hm {
        //         eprintln!("[{k}] {kk:?}:{v:?}");
        //     }
        // }
    }
    // dbg!(&hm);
    let rs = hm.into_values().next().unwrap().1;
    println!("{rs}");
}
