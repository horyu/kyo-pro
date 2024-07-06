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
        mut s: Chars,
        mut t: Chars,
    };
    let nn = n + 2;
    s.extend(['.'; 2]);
    t.extend(['.'; 2]);
    let mut hs = HashSet::new();
    let mut qq = VecDeque::new();
    hs.insert(s.clone());
    qq.push_back((s, 0));
    while let Some((vv, cnt)) = qq.pop_front() {
        if vv == t {
            println!("{cnt}");
            return;
        }
        let empty_pos = vv.iter().position(|&c| c == '.').unwrap();
        for i in 0..=n {
            if vv[i] == '.' || vv[i + 1] == '.' {
                continue;
            }
            let mut vv = vv.clone();
            vv.swap(i, empty_pos);
            vv.swap(i + 1, empty_pos + 1);
            if hs.insert(vv.clone()) {
                qq.push_back((vv, cnt + 1));
            }
        }
    }
    println!("-1");
}
