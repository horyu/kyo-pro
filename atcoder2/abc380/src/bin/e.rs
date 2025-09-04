#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
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
    };
    // left => (color, right)
    let mut btm = BTreeMap::from_iter((1..=n).map(|i| (i, (i, i))));
    let mut cnts = vec![1; n + 1];
    for _ in 0..q {
        input! { t: u32 };
        if t == 1 {
            input! { x: usize, nc: usize };
            let (&cl, &(cc, cr)) = btm.range(..=x).next_back().unwrap();
            if cc == nc {
                continue;
            }
            let size = cr - cl + 1;
            cnts[cc] -= size;
            cnts[nc] += size;
            btm.entry(cl).and_modify(|v| v.0 = nc);
            if let Some((&rl, &(rc, rr))) = btm.range((x + 1)..).next() {
                if nc == rc {
                    btm.remove(&rl);
                    btm.entry(cl).and_modify(|v| v.1 = rr);
                }
            }
            if let Some((&ll, &(lc, lr))) = btm.range(..cl).next_back() {
                if lc == nc {
                    let r = btm.remove(&cl).unwrap().1;
                    btm.entry(ll).and_modify(|v| v.1 = r);
                }
            }
            // eprintln!("{cnts:?}");
            // eprintln!("{btm:?}");
        } else {
            input! { c: usize };
            let rs = cnts[c];
            println!("{rs}");
        }
    }
}
