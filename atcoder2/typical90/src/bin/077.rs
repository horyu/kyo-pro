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
        t: isize,
        aabb: [(isize, isize); n],
        xxyy: [(isize, isize); n],
    };
    // n=2
    // SiijjG
    let xy2j = xxyy
        .iter()
        .copied()
        .enumerate()
        .map(|(j, (x, y))| ((x, y), j))
        .collect::<HashMap<_, _>>();
    let mut ij2k = HashMap::new();
    let mut mg = ac_library::maxflow::MfGraph::new(2 * n + 2);
    for (i, (a, b)) in aabb.iter().copied().enumerate() {
        mg.add_edge(0, i + 1, 1);
        mg.add_edge(n + 1 + i, 2 * n + 1, 1);
        for (k, (dx, dy)) in [
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ]
        .iter()
        .copied()
        .enumerate()
        {
            let (aa, bb) = (a + t * dx, b + t * dy);
            if let Some(&j) = xy2j.get(&(aa, bb)) {
                // eprintln!("{} -> {}", i + 1, j);
                mg.add_edge(i + 1, j + n + 1, 1);
                ij2k.insert((i, j), k);
            }
        }
    }
    if n == mg.flow(0, 2 * n + 1) {
        println!("Yes");
        let mut rs = vec![0; n];
        for e in mg.edges() {
            if e.flow == 1 && (1..=n).contains(&e.from) {
                let i = e.from - 1;
                let j = e.to - n - 1;
                rs[i] = ij2k[&(i, j)] + 1;
            }
        }
        println!("{}", rs.into_iter().join(" "));
    } else {
        println!("No");
    }
}
