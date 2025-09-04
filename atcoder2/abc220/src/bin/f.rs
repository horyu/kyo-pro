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
        uuvv: [(Usize1, Usize1); n - 1]
    };
    let mut g = vec![vec![]; n];
    for (u, v) in uuvv.iter().copied() {
        g[u].push(v);
        g[v].push(u);
    }
    let mut memo = HashMap::new();
    for i in 0..n {
        let rs = f(i, !0, &mut memo, &g).1;
        println!("{rs}");
    }
    // for (k, v) in memo {
    //     eprintln!("{k:?}: {v:?}");
    // }
}

fn f(
    i: usize,
    from: usize,
    memo: &mut HashMap<(usize, usize), (usize, usize)>,
    g: &[Vec<usize>],
) -> (usize, usize) {
    if let Some(&x) = memo.get(&(i, from)) {
        return x;
    }
    // g(a, b) = bから見てaを根とした部分木の (頂点数, 経路合計)
    // f(a, b) = bから見てaを根とした部分木のaを含まない (頂点数, 経路合計)
    // g(a, b) = (f.0 + 1, f.0 + f.1 + 1)
    // g(i, !0) = sum(隣接頂点j) {g(j, i)}
    // f(a, b) = g(a, !0) - g(b, a)
    let mut cnt = 0;
    let mut sum = 0;
    for j in g[i].iter().copied() {
        if j != from {
            if let (Some(&x), Some(&y)) = (memo.get(&(j, !0)), memo.get(&(i, j))) {
                let z = (y.0 + 1, y.0 + y.1 + 1);
                let c = x.0 - z.0;
                let s = x.1 - z.1;
                cnt += c + 1;
                sum += s + c + 1;
                // eprintln!("u {i}-{j} ({from}):({c},{s})");
            } else {
                let y = f(j, i, memo, g);
                cnt += y.0 + 1;
                sum += y.0 + y.1 + 1;
                // eprintln!("d {i}-{j} ({from}): {} {}", y.0 + 1, y.0 + y.1 + 1);
            }
        }
    }
    // eprintln!("ins: {:?} {:?}", (i, from), (cnt, sum));
    memo.insert((i, from), (cnt, sum));
    (cnt, sum)
}
