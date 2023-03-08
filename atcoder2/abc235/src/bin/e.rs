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
        q: usize,
        mut aabbcc: [(Usize1, Usize1, usize); m],
        uuvvww: [(Usize1, Usize1, usize); q],
    };
    aabbcc.sort_unstable_by_key(|abc| abc.2);
    aabbcc.dedup_by_key(|abc| (abc.0, abc.1));
    let mut uf = UnionFind::new(n);
    let mut ttff = vec![false; q];
    let mut jjuuvvww = uuvvww
        .into_iter()
        .enumerate()
        .sorted_unstable_by_key(|juvw| juvw.1 .2)
        .peekable();
    // for (j, uvw) in jjuuvvww {
    //     eprintln!("{j} {uvw:?}");
    // }
    // return;
    for (a, b, c) in aabbcc.iter().copied() {
        while let Some(&(j, (u, v, w))) = jjuuvvww.peek() {
            if w < c {
                ttff[j] = !uf.equiv(u, v);
                jjuuvvww.next();
            } else {
                break;
            }
        }
        uf.union(a, b);
    }
    for tf in ttff {
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
