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
        uuvv: [(Usize1, Usize1); m],
        k: usize,
        ppdd: [(Usize1, usize); k],
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ddd = vec![BTreeMap::new(); n];
    for i in 0..n {
        let mut qq = VecDeque::new();
        qq.push_back((i, 0));
        let mut pushed = vec![false; n];
        pushed[i] = true;
        while let Some((qi, qd)) = qq.pop_front() {
            ddd[i].entry(qd).or_insert_with(Vec::new).push(qi);
            for j in g[qi].iter().copied() {
                if !pushed[j] {
                    pushed[j] = true;
                    qq.push_back((j, qd + 1));
                }
            }
        }
    }
    'b: {
        let mut rs = vec![2; n];
        for (p, d) in ppdd.iter().copied() {
            if d == 0 {
                rs[p] = 1;
            } else {
                for &i in ddd[p].range(..d).flat_map(|x| x.1) {
                    rs[i] = 0;
                }
            }
        }
        for rs in rs.iter_mut() {
            if *rs == 2 {
                *rs = 1;
            }
        }
        for (p, d) in ppdd.iter().copied() {
            if 0 == d {
                if rs[p] == 0 {
                    break 'b;
                }
            } else {
                for &i in ddd[p].range(..d).flat_map(|x| x.1) {
                    if rs[i] == 1 {
                        break 'b;
                    }
                }
                if let Some(vv) = ddd[p].get(&d) {
                    if vv.iter().all(|&v| rs[v] == 0) {
                        break 'b;
                    }
                } else {
                    break 'b;
                }
            }
        }
        if rs.iter().all(|&rs| rs == 0) {
            break 'b;
        }
        println!("Yes");
        println!("{}", rs.iter().join(""));
        return;
    };
    println!("No");
}
