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
        n: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut vv = vec![];
    let mut base = vec![];
    for l in 0..n {
        if 0 < l {
            base.push(aa[l - 1]);
        }
        for r in l..n {
            let mut tmp = base.clone();
            tmp.extend(aa[l..=r].iter().rev());
            tmp.extend(aa[(r + 1)..n].iter());
            vv.push(tmp);
        }
    }
    vv.sort_unstable();
    // for (i, v) in vv.into_iter().enumerate() {
    //     eprintln!("{i:2}: {}", v.iter().join(" "));
    // }
    let rs = vv[k - 1].iter().join(" ");
    println!("{rs}");
}
