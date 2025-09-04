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
        aa: [usize; n],
        q: usize,
    };
    // k => (pre_value, next_value)
    let mut hm = HashMap::new();
    for i in 0..n {
        let a = aa[i];
        let pre = if i == 0 { !0 } else { aa[i - 1] };
        let next = if i == n - 1 { !0 } else { aa[i + 1] };
        hm.insert(a, (pre, next));
    }
    for _ in 0..q {
        // eprintln!("{hm:?}");
        input!(t: usize);
        if t == 1 {
            input! {x: usize, y: usize};
            let next;
            {
                let pn = hm.get_mut(&x).unwrap();
                next = pn.1;
                pn.1 = y;
            }
            if let Some(pn) = hm.get_mut(&next) {
                pn.0 = y;
            }
            hm.insert(y, (x, next));
            continue;
        }
        input! {x: usize};
        let (pre, next) = hm.remove(&x).unwrap();
        if let Some(pn) = hm.get_mut(&pre) {
            pn.1 = next;
        }
        if let Some(pn) = hm.get_mut(&next) {
            pn.0 = pre;
        }
    }
    let mut qq = VecDeque::new();
    let base = *hm.iter().next().unwrap().0;
    let mut cur = base;
    while let Some(&(pre, _)) = hm.get(&cur) {
        if pre == !0 {
            break;
        }
        qq.push_front(pre);
        cur = pre;
    }
    qq.push_back(base);
    cur = base;
    while let Some(&(_, next)) = hm.get(&cur) {
        if next == !0 {
            break;
        }
        qq.push_back(next);
        cur = next;
    }
    let rs = qq.into_iter().join(" ");
    println!("{rs}");
}
