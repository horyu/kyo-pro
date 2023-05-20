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
    };
    let mut aa = vec![];
    let mut ss = vec![];
    let mut num_to_i = vec![vec![]; m];
    for i in 0..n {
        input! {a: usize, s: [Usize1; a]};
        for x in s.iter().copied() {
            num_to_i[x].push(i);
        }
        aa.push(a);
        ss.push(s);
    }
    let mut qq = VecDeque::new();
    let mut pushed = vec![false; m];
    for i in num_to_i[0].iter().copied() {
        qq.push_back((0, i, 0));
    }
    // pushed[0] = true;
    while let Some((qnum, qi, qd)) = qq.pop_front() {
        for num in ss[qi].iter().copied() {
            if num == m - 1 {
                println!("{qd}");
                return;
            }
            if !pushed[num] {
                pushed[num] = true;
                for j in num_to_i[num].iter().copied() {
                    if j != qi {
                        qq.push_back((num, j, qd + 1));
                    }
                }
            }
        }
    }
    println!("-1");
}
