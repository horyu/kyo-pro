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
    let mut stdin =
        proconio::source::line::LineSource::new(std::io::BufReader::new(std::io::stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
    input! {
        n: usize,
        uuvv: [(Usize1, Usize1); n - 1],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    // https://atcoder.jp/contests/abc398/editorial/12483
    // 木を二部グラフに分ける
    let mut cc = vec![!0; n];
    let mut stack = vec![(0, 0)];
    while let Some((u, c)) = stack.pop() {
        cc[u] = c;
        for &v in g[u].iter() {
            if cc[v] == !0 {
                stack.push((v, 1 ^ c));
            }
        }
    }
    // eprintln!("{cc:?}");
    let (aa, bb) = (0..n).partition::<Vec<_>, _>(|&i| cc[i] == 0);
    // eprintln!("{aa:?}\n{bb:?}");
    // 奇閉路候補
    let mut bts = BTreeSet::new();
    for (a, b) in iproduct!(aa, bb) {
        bts.insert((a.min(b), b.max(a)));
    }
    for (u, v) in uuvv.iter().copied() {
        bts.remove(&(u, v));
    }
    if bts.len() % 2 == 1 {
        println!("First");
        let (a, b) = bts.pop_first().unwrap();
        println!("{} {}", a + 1, b + 1);
    } else {
        println!("Second");
    }
    loop {
        input! {
            i: isize,
            j: isize,
        };
        if i == -1 && j == -1 {
            return;
        }
        let i = i as usize - 1;
        let j = j as usize - 1;
        bts.remove(&(i, j));
        let (a, b) = bts.pop_first().unwrap();
        println!("{} {}", a + 1, b + 1);
    }
}
