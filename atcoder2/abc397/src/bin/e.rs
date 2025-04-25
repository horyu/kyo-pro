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

/*
木をn本の頂点数kのパスに分解できるか
切り方は 2通り
- 直線の端を切る
- Y字の真ん中を含めて切る

方法
- 全ての葉ノードを集める
- 各葉ノードに対して、長さkのパスを作って間に分岐がなければ切る & 切断したノードから距離k以内の葉ノードを再処理対象に加える
// ここで葉ノードは全て流さk未満となる
- 各葉ノードに対して分岐を持つ内部ノードへの距離を登録し、パスの長さの和をkにできたら切断する。切断したノードから距離k以内の葉ノードを再処理対象に加える
*/
fn main() {
    input! {
        n: usize,
        k: usize,
        uuvv: [(Usize1, Usize1); n * k - 1],
    };
    let mut g = vec![HashSet::new(); n * k];
    for (u, v) in uuvv.iter().copied() {
        g[u].insert(v);
        g[v].insert(u);
    }
    let mut hs = HashSet::<_>::from_iter(0..n);
    let mut ll = HashSet::new();
    while !hs.is_empty() {
        let len = hs.len();
        let new_ll = hs.iter().copied()
            .filter(|&x| g[x].len() == 1)
            .collect_vec();
        if new_ll.is_empty() {
            println!("No");
            return;
        }
        for new_l in new_ll {
            hs.remove(&new_l);
            ll.insert(new_l);
        }


        if len == hs.len() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
