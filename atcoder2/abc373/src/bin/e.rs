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
    let sum = aa.iter().sum::<usize>();
    // 投票数が小さい順にソートし後ろから考える
    let mut iiaa = aa.into_iter().enumerate().sorted_unstable_by_key(|ia| ia.1).collect_vec();
    let mut kk = k;
    let mut mm = m;
    let mut rrss = vec![-1; n];
    // 当選確実者を除く
    while let Some((i, a)) = iiaa.last().copied() {
        if kk.div_ceil(mm + 1) <= a {
            mm = mm.saturating_sub(1);
            kk -= a;
            iiaa.pop();
            rrss[i] = 0;
            continue;
        }
        break;
    }
    dbg!(sum);
    dbg!(kk, mm);
    eprintln!("{rrss:?}");

    // 得票数上位mm位までが当選する（同数の場合は全員当選）
    // 人iを当選確実とするには上位mm人と人iを考えれば良い
    // 考慮対象以外は総数から引いて考えた方が良い？

    // println!("{rs}");
}
