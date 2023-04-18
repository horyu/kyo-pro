#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        aa: [usize; n],
        bb: [usize; m],
    };
    let rs = f(&aa, &bb, &mut HashMap::new());
    println!("{rs}");
}

fn f(cc: &[usize], dd: &[usize], memo: &mut HashMap<(usize, usize), usize>) -> usize {
    let clen = cc.len();
    let dlen = dd.len();
    if let Some(rs) = memo.get(&(clen, dlen)).copied() {
        return rs;
    }
    let rs = if clen == 0 || dlen == 0 {
        clen.max(dlen)
    } else {
        ((cc[0] != dd[0]) as usize + f(&cc[1..], &dd[1..], memo))
            .min(1 + f(cc, &dd[1..], memo))
            .min(1 + f(&cc[1..], dd, memo))
    };
    memo.insert((clen, dlen), rs);
    rs
}
