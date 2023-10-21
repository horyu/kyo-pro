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
        k: usize,
        aa: [usize; n],
    };
    let mut yobi = BTreeSet::new();
    for (i, a) in aa[0..(m - 1)].iter().copied().enumerate() {
        yobi.insert((a, i));
    }
    let mut main = BTreeSet::new();
    let mut sum = 0;
    let mut rs = vec![];
    for (r, ar) in aa.iter().copied().enumerate().skip(m - 1) {
        yobi.insert((ar, r));
        while main.len() < k {
            if let Some((ai, i)) = yobi.pop_first() {
                sum += ai;
                main.insert((ai, i));
            }
        }

        if let Some((main_max, _)) = main.last().copied() {
            if let Some((yobi_min, _)) = yobi.first().copied() {
                if yobi_min < main_max {
                    sum = sum + yobi_min - main_max;
                    let max = main.pop_last().unwrap();
                    let min = yobi.pop_first().unwrap();
                    main.insert(min);
                    yobi.insert(max);
                }
            }
        }
        rs.push(sum);
        // eprintln!("main:{}", main.iter().map(|kv| format!("{kv:?}")).join(" "));
        // eprintln!("yobi:{}", yobi.iter().map(|kv| format!("{kv:?}")).join(" "));

        let rm_value = (aa[r + 1 - m], r + 1 - m);
        if main.remove(&rm_value) {
            sum -= rm_value.0;
        };
        yobi.remove(&rm_value);
    }
    println!("{}", rs.iter().join(" "));
}
