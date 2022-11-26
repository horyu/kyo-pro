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
        n: isize,
    };
    if n == 0 {
        println!("0");
        return;
    }
    let mut pp = vec![];
    let mut nn = vec![];
    for i in 0u32..34 {
        let vv = if i.is_even() { &mut pp } else { &mut nn };
        let w = (-2isize).pow(i);
        let b = 1usize << i;
        for j in 0..vv.len() {
            let (ww, bb) = vv[j];
            vv.push((ww + w, bb | b));
        }
        vv.push((w, b));
        // eprintln!("{w:12} {b:34b}");
    }
    let pp: HashMap<_, _> = pp.into_iter().collect();
    let nn: HashMap<_, _> = nn.into_iter().collect();
    for hm in [&pp, &nn] {
        if let Some(rs) = hm.get(&n) {
            println!("{rs:b}");
            return;
        }
    }
    for (pw, pb) in pp {
        if let Some(nb) = nn.get(&(n - pw)) {
            let rs = pb | nb;
            println!("{rs:b}");
            return;
        }
    }
}
