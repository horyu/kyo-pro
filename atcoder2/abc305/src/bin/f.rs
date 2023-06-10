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
    let stdin = std::io::stdin();
    let mut s = String::new();

    stdin.read_line(&mut s).unwrap();
    let nm = s
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    s.clear();
    let n = nm[0];
    let m = nm[1];

    stdin.read_line(&mut s).unwrap();
    let vv = s
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec();
    s.clear();
    let mut crr = 1;

    let mut g = vec![vec![]; n + 1];
    g[crr] = vv;
    let mut ff = vec![!0; n + 1];

    while !g[crr].contains(&n) {
        let mut next = ff[crr];
        for &i in &g[crr] {
            if ff[i] == !0 {
                next = i;
                break;
            }
        }
        println!("{next}");

        if ff[next] == !0 {
            ff[next] = crr;
        }
        crr = next;

        stdin.read_line(&mut s).unwrap();
        let vv = s
            .split_ascii_whitespace()
            .skip(1)
            .map(|s| s.parse::<usize>().unwrap())
            .collect_vec();
        s.clear();
        g[crr] = vv;
    }

    println!("{n}");
}
