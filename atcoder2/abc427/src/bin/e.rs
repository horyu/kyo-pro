#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut ss: [Chars; h],
    };
    let (tx, ty) = ss
        .iter()
        .enumerate()
        .find_map(|(i, s)| {
            s.iter()
                .enumerate()
                .find_map(|(j, &c)| (c == 'T').then_some((i, j)))
        })
        .unwrap();
    ss[tx][ty] = '.';
    let mut qq = VecDeque::new();
    qq.push_back((ss.clone(), 1));
    let mut checked = HashSet::new();
    checked.insert(ss);
    let f = |ss: &Vec<Vec<char>>| -> Vec<Vec<Vec<char>>> {
        let mut res = vec![];
        // 上にずらす
        {
            let mut vv = ss.clone();
            vv.rotate_left(1);
            vv[h - 1].fill('.');
            res.push(vv);
        }
        // 下にずらす
        {
            let mut vv = ss.clone();
            vv.rotate_right(1);
            vv[0].fill('.');
            res.push(vv);
        }
        // 左にずらす
        {
            let mut vv = ss.clone();
            for s in vv.iter_mut() {
                s.rotate_left(1);
                s[w - 1] = '.';
            }
            res.push(vv);
        }
        // 右にずらす
        {
            let mut vv = ss.clone();
            for s in vv.iter_mut() {
                s.rotate_right(1);
                s[0] = '.';
            }
            res.push(vv);
        }
        res.into_iter().filter(|vv| vv[tx][ty] == '.').collect()
    };
    while let Some((ss, d)) = qq.pop_front() {
        for vv in f(&ss) {
            if checked.insert(vv.clone()) {
                if vv.iter().flatten().all(|&c| c == '.') {
                    println!("{d}");
                    return;
                }
                qq.push_back((vv, d + 1));
            }
        }
    }
    println!("-1");
}
