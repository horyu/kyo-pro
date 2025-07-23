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
        q: usize,
    };
    // ss[i] = (parent, string)
    let mut ss = vec![(!0, String::new())];
    let mut pp = vec![0; n];
    let mut s = 0;
    for qi in 0..q {
        input! {t: usize, p: Usize1};
        match t {
            1 => {
                // eprintln!("{qi} {t} {p}->{s}");
                pp[p] = s;
            }
            2 => {
                input! {s: String};
                let par = pp[p];
                pp[p] = ss.len();
                ss.push((par, s));
            }
            _ => {
                // eprintln!("{qi} {t} {p}->{s}");
                s = pp[p];
            }
        }
    }
    let mut qq = VecDeque::new();
    let mut cur = s;
    while cur != !0 {
        let (par, s) = std::mem::take(&mut ss[cur]);
        qq.push_front(s);
        cur = par;
    }
    let rs = qq.into_iter().join("");
    println!("{rs}");
}
