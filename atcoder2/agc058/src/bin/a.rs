#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_log, int_roundings)]
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        pp: [usize; 2 * n],
    };
    let swap_even = |vv: &mut Vec<usize>, kk: &mut Vec<usize>| {
        for i in (0..(2 * n - 1)).step_by(2) {
            if vv[i] > vv[i + 1] {
                kk.push(i);
                vv.swap(i, i + 1);
            }
        }
    };
    let swap_odd = |vv: &mut Vec<usize>, kk: &mut Vec<usize>| {
        for i in (1..(2 * n - 1)).step_by(2) {
            if vv[i] < vv[i + 1] {
                kk.push(i);
                vv.swap(i, i + 1);
            }
        }
    };

    let mut qq = pp.clone();
    let mut kk = vec![];
    swap_even(&mut qq, &mut kk);
    swap_odd(&mut qq, &mut kk);
    if kk.len() <= n {
        if kk.is_empty() {
            println!("0");
        } else {
            println!("{}\n{}", kk.len(), kk.into_iter().map(|k| k + 1).join(" "));
        }
        return;
    };

    qq = pp;
    kk.clear();
    swap_odd(&mut qq, &mut kk);
    swap_even(&mut qq, &mut kk);
    if kk.is_empty() {
        println!("0")
    } else {
        println!("{}\n{}", kk.len(), kk.into_iter().map(|k| k + 1).join(" "));
    }
}
