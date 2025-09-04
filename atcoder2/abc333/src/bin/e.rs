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
        ttxx: [(usize, Usize1); n],
    };
    // aaa[i] = ポーションiの位置配列
    // bab[i] = ポーションiの位置配列
    let mut aaa = vec![vec![]; n];
    let mut bbb = vec![vec![]; n];
    for (i, (t, x)) in ttxx.iter().copied().enumerate() {
        if t == 1 {
            aaa[x].push(i);
        } else {
            bbb[x].push(i);
        }
    }
    // vv[i] = 1:位置iでポーションを拾う -1:位置iでポーションを使う
    let mut vv = vec![0; n];
    // bの位置にできるだけ近く前に出現するaを拾う
    for (mut aa, mut bb) in izip!(aaa, bbb) {
        while let Some(b) = bb.pop() {
            vv[b] = -1;
            let mut ok = false;
            while let Some(a) = aa.pop() {
                if b < a {
                    continue;
                }
                vv[a] = 1;
                ok = true;
                break;
            }
            if !ok {
                println!("-1");
                return;
            }
        }
    }

    let k = vv.iter().copied().cumsum::<i32>().max().unwrap();
    let ww = izip!(vv, ttxx)
        .filter_map(|(v, (t, x))| (t == 1).then_some(v))
        .join(" ");
    println!("{k}");
    println!("{ww}");
}
