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
        k: usize,
        d: isize,
        aa: [isize; n],
    };
    let mut rs = -1;
    let mut hm: HashMap<(isize, usize), isize> = HashMap::new();
    hm.insert((0, 0), 0);
    for a in aa {
        let mut new_hm = hm.clone();
        for ((km, kc), s) in hm {
            let kkm = (km + a) % d;
            let kkc = kc + 1;
            if kkc == k {
                if kkm == 0 {
                    rs = rs.max(s + a);
                }
            } else {
                let e = new_hm.entry((kkm, kkc)).or_default();
                *e = (*e).max(s + a);
            }
        }
        hm = new_hm;
    }
    println!("{rs}");
}
