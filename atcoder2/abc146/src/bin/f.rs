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
        m: usize,
        s: Chars,
    };
    if n <= m {
        println!("{n}");
        return;
    }

    let mut ng = vec![false; n + 1];
    for (i, c) in s.into_iter().enumerate() {
        if c == '1' {
            ng[i] = true;
        }
    }
    let mut vv = vec![!0; n + 1];
    vv[n] = 0;
    let mut bts = BTreeSet::new();
    let to = |cnt: usize, i: usize| cnt * 1000000 + n + 10 - i;
    let from = |num: usize| (num / 1000000, n + 10 - num % 1000000);
    bts.insert(to(0, n));
    // 右から左に見ていく
    for i in (0..n).rev() {
        let j = i + m + 1;
        if j <= n && vv[j] != !0 {
            bts.remove(&to(vv[j], j));
        }
        if ng[i] {
            continue;
        }
        if let Some(num) = bts.iter().min().copied() {
            let (cnt, _j) = from(num);
            vv[i] = cnt + 1;
            bts.insert(to(cnt + 1, i));
        }
    }
    // for i in 0..n {
    //     if vv[i] == !0 {
    //         eprintln!("{i}:{}  -", ng[i]);
    //     } else {
    //         eprintln!("{i}:{} {}", ng[i], vv[i]);
    //     }
    // }
    if vv[0] == !0 {
        println!("-1");
    } else {
        // 辞書順最小にするため最初の移動を最短にする
        let mut f = 0;
        let mut rs = vec![];
        for _ in 0..vv[0] {
            let new_f = ((f + 1)..=(f + m)).find(|&j| vv[j] == vv[f] - 1).unwrap();
            rs.push(new_f - f);
            f = new_f;
        }
        println!("{}", rs.iter().join(" "));
    }
}
