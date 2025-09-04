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
        ss: [Chars; n],
    };
    let mut block = vec![vec![false; n]; n];
    let mut starts = vec![];
    for (i, s) in ss.iter().enumerate() {
        for (j, c) in s.iter().copied().enumerate() {
            match c {
                '#' => block[i][j] = true,
                'P' => {
                    starts.push(i as i32);
                    starts.push(j as i32);
                }
                _ => (),
            }
        }
    }
    let mut qq = VecDeque::new();
    let mut pushed = HashSet::new();
    let arr: [i32; 4] = starts.try_into().unwrap();
    qq.push_back((arr, 0));
    pushed.insert(arr);
    let dxdy = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let n = n as i32;
    while let Some((arr, qd)) = qq.pop_front() {
        for (dx, dy) in dxdy {
            let mut brr = arr;
            for (x, y) in brr.iter_mut().tuples() {
                *x += dx;
                *y += dy;
            }
            for v in brr.iter_mut() {
                if *v < 0 {
                    *v = 0;
                } else if n <= *v {
                    *v = n - 1;
                }
            }
            for (x, y) in brr.iter_mut().tuples() {
                if block[*x as usize][*y as usize] {
                    *x -= dx;
                    *y -= dy;
                }
            }
            if brr[0..2] == brr[2..] {
                println!("{}", qd + 1);
                return;
            }
            if pushed.insert(brr) {
                qq.push_back((brr, qd + 1));
            }
        }
    }
    println!("-1");
}
