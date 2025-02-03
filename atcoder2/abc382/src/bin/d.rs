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
    };
    // let n = 12; let m = 120;
    let mut rs = vec![];
    let mut vv = vec![];
    dfs(&mut rs, &mut vv, n, m);
    println!("{}", rs.len());
    let rs = rs.iter().join("\n");
    println!("{rs}");
}

fn dfs(rs: &mut Vec<String>, vv: &mut Vec<usize>, n: usize, m: usize) -> bool {
    if vv.len() == n {
        // if vv.iter().any(|&v| m < v) || vv.iter().tuple_windows().any(|(&a, &b)| !(a + 10 <= b)) {
        //     panic!("{vv:?}");
        // }
        rs.push(vv.iter().join(" "));
        return false;
    }
    if let Some(last) = vv.last().copied() {
        if m < last + 10 {
            return true;
        }
        for i in (last + 10)..=(last + 20).min(m) {
            vv.push(i);
            let tf = dfs(rs, vv, n, m);
            vv.pop();
            if tf {
                break;
            }
        }
    } else {
        for i in 1..=10 {
            vv.push(i);
            dfs(rs, vv, n, m);
            vv.pop();
        }
    }
    false
}
