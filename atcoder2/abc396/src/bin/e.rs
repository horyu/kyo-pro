#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
#![feature(map_try_insert)]
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
        xxyyzz: [(Usize1, Usize1, usize); m],
    };

    let mut g = vec![vec![]; n];
    let mut dsu = ac_library::Dsu::new(n);
    for (x, y, z) in xxyyzz.iter().copied() {
        g[x].push((y, z));
        g[y].push((x, z));
        dsu.merge(x, y);
    }
    let mut aa = vec![0; n];
    for ii in dsu.groups() {
        // bit 単位で xor が成立するか確かめる
        for d in 0..32 {
            let mut pre_sum = !0;
            for b in [0, 1] {
                let mut qq = VecDeque::new();
                let mut hm = HashMap::new();
                qq.push_back((ii[0], b));
                hm.insert(ii[0], b);
                let mut sum = b;
                while let Some((i, b)) = qq.pop_front() {
                    for (j, z) in g[i].iter().copied() {
                        if hm.contains_key(&j) {
                            continue;
                        }
                        let zz = (z >> d) & 1;
                        let bb = b ^ zz;
                        if hm.try_insert(j, bb).is_ok() {
                            qq.push_back((j, bb));
                            sum += bb;
                        }
                    }
                }
                if sum < pre_sum {
                    pre_sum = sum;
                    for (i, b) in hm {
                        // aa[i] の d ビット目を b にする
                        aa[i] &= !(1 << d);
                        aa[i] |= b << d;
                    }
                }
            }
        }
    }
    if xxyyzz.iter().copied().all(|(x, y, z)| aa[x] ^ aa[y] == z) {
        let rs = aa.iter().join(" ");
        println!("{rs}");
    } else {
        println!("-1");
    }
}
