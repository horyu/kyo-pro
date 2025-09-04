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
        m: usize,
        xx: [Usize1; m],
    };
    let mut imos = vec![0; n];
    for (xi, xj) in xx.iter().copied().tuple_windows() {
        let xl = xi.min(xj);
        let xr = xi.max(xj);
        // l→0→(n-1)→r
        let dl = xl + 1 + (n - 1 - xr);
        // l→r
        let dr = xr - xl;

        let d = dl as isize - dr as isize;
        match d.cmp(&0) {
            // l→rの区間は|d|だけ長い
            Ordering::Less => {
                imos[xl] += d.abs();
                imos[xr] -= d.abs();
            }
            // どちら側の橋を落としても距離が変わらない
            Ordering::Equal => (),
            Ordering::Greater => {
                imos[0] += d;
                imos[xl] -= d;
                imos[xr] += d;
            }
        }
    }
    for i in 1..n {
        imos[i] += imos[i - 1];
    }
    let remove_pos = imos.iter().copied().position_max().unwrap_or_default();
    let mut rs = 0;
    for (xi, xj) in xx.iter().copied().tuple_windows() {
        let xl = xi.min(xj);
        let xr = xi.max(xj);
        if (xl..xr).contains(&remove_pos) {
            rs += xl + 1 + (n - 1 - xr);
        } else {
            rs += xr - xl;
        }
    }
    println!("{rs}");
}
