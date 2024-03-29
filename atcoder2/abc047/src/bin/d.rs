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
        t: usize,
        aa: [isize; n],
    };
    let mut btm = BTreeMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        btm.entry(a).or_insert_with(VecDeque::new).push_back(i);
    }
    let mut diff = 0;
    let mut sml = HashMap::new();
    let mut big = HashMap::new();
    for (i, a) in aa.iter().copied().enumerate() {
        if let Some(vd) = btm.get_mut(&a) {
            vd.pop_front();
            if vd.is_empty() {
                btm.remove(&a);
            }
        }
        if let Some((&b, jj)) = btm.iter().max() {
            if a < b {
                match (b - a).cmp(&diff) {
                    std::cmp::Ordering::Less => (),
                    std::cmp::Ordering::Equal => {
                        let e = sml.entry(a).or_insert(0);
                        if *e == 0 {
                            big.insert(b, jj.len());
                        }
                        *e += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        diff = b - a;
                        sml.clear();
                        sml.insert(a, 1);
                        big.clear();
                        big.insert(b, jj.len());
                    }
                }
            }
        }
    }
    let mut rs = 0;
    for (sk, sc) in sml {
        let bc = big.get(&(sk + diff)).copied().unwrap();
        rs += sc.min(bc);
    }
    println!("{rs}");
}

fn _main() {
    input! {
        n: usize,
        t: usize,
        aa: [isize; n],
    };
    // 現在地と左側の (最も安い価格, 価格で取引できる街の数)
    let mut ll = vec![(0, 0); n];
    // 現在地と右側の (最も高い価格, 価格で取引できる街の数)
    let mut rr = vec![(0, 0); n];

    ll[0] = (aa[0], 1);
    for i in 1..n {
        ll[i] = match aa[i].cmp(&ll[i - 1].0) {
            std::cmp::Ordering::Less => (aa[i], 1),
            std::cmp::Ordering::Equal => (ll[i - 1].0, ll[i - 1].1 + 1),
            std::cmp::Ordering::Greater => ll[i - 1],
        };
    }
    rr[n - 1] = (aa[n - 1], 1);
    for i in (0..(n - 1)).rev() {
        rr[i] = match aa[i].cmp(&rr[i + 1].0) {
            std::cmp::Ordering::Less => rr[i + 1],
            std::cmp::Ordering::Equal => (rr[i + 1].0, rr[i + 1].1 + 1),
            std::cmp::Ordering::Greater => (aa[i], 1),
        };
    }

    let mut max_diff = 0;
    let mut vv = vec![];
    for i in 0..(n - 1) {
        let diff = rr[i + 1].0 - ll[i].0;
        match diff.cmp(&max_diff) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => {
                vv.push(i);
            }
            std::cmp::Ordering::Greater => {
                max_diff = diff;
                vv = vec![i];
            }
        }
    }

    let mut rs = 0;
    for (_k, g) in vv.into_iter().group_by(|&v| ll[v].0).into_iter() {
        let mut lcnt = 0;
        let mut rcnt = 0;
        for v in g {
            lcnt = lcnt.max(ll[v].1);
            rcnt = rcnt.max(rr[v + 1].1);
        }
        rs += lcnt.min(rcnt);
    }
    println!("{rs}");
}
