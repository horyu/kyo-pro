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
        aa: [Usize1; m],
    };
    // 0が座標aにあるとき、有効な移動候補2つ (移動のインデックス, 次の座標)
    let mut abtm = vec![BTreeMap::new(); n];
    for (j, a) in aa.iter().copied().enumerate() {
        abtm[a].insert(j, vec![(j, a + 1)]);
        abtm[a + 1].insert(j, vec![(j, a)]);
    }
    for (j, a) in aa.iter().copied().enumerate() {
        if let Some((_, vv)) = abtm[a].range_mut(..j).next_back() {
            vv.push((j, a + 1));
        }
        if let Some((_, vv)) = abtm[a + 1].range_mut(..j).next_back() {
            vv.push((j, a));
        }
    }
    for btm in abtm.iter_mut() {
        for vv in btm.values_mut() {
            vv.sort_unstable();
        }
    }
    let mut start_index = 0;
    let mut start_pos = 0;
    let mut memo = HashMap::new();
    for i in 0..m {
        let mut index = start_index;
        let mut pos = start_pos;
        let mut history = vec![];
        while let Some((_, jjpp)) = abtm[pos].range(index..).next() {
            if let Some((j, p)) = jjpp.iter().copied().find(|jp| jp.0 != i) {
                if i < index {
                    if let Some(&rs) = memo.get(&(j, p)) {
                        pos = rs;
                        break;
                    }
                }
                index = j + 1;
                pos = p;
                if j < i {
                    start_index = index;
                    start_pos = pos;
                } else {
                    history.push((j, p));
                }
                continue;
            }
            break;
        }
        memo.extend(history.into_iter().map(|jp| (jp, pos)));
        println!("{}", pos + 1);
    }
}
