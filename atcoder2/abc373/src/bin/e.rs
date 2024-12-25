#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use itertools::{chain, iproduct, izip, Itertools};
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
        k: isize,
        aa: [isize; n],
    };
    // https://atcoder.jp/contests/abc373/editorial/11044
    // https://atcoder.jp/contests/abc373/submissions/59102624
    if n == m {
        let rs = vec![0; n].iter().join(" ");
        println!("{rs}");
        return;
    }

    let aaii = aa
        .iter()
        .copied()
        .zip(0usize..)
        .sorted_unstable()
        .collect_vec();
    let mut poss = vec![0; n];
    for (i, (_, x)) in aaii.iter().copied().enumerate() {
        poss[x] = i;
    }

    let mut ss = vec![0; n + 1];
    for (i, ai) in aaii.iter().copied().enumerate() {
        ss[i + 1] = ss[i] + ai.0;
    }
    let r = k - ss[n];
    let mut rrss = vec![-1; n];

    // 候補iが追加でx票とったとき，当選が確定するか
    let f = |i: usize, x: isize| -> bool {
        let sum_i = aa[i] + x;
        let r_rem = r - x;

        let idx = poss[i];
        // 得票が sum_i 以上の候補の数
        let over = aaii.partition_point(|&ai| ai.0 <= sum_i);
        let over_cnt = n - over;
        if m <= over_cnt {
            return false;
        }
        // 当選できる残り人数
        let m_rem = m - over_cnt;

        let mut lower = over - m_rem;
        // 候補iが含まれているとき範囲を広める
        if lower <= idx {
            lower -= 1;
        }

        let mut rem_total = ss[over] - ss[lower];
        if lower <= idx {
            rem_total -= aaii[idx].0;
        }
        let total_cnt = (sum_i + 1) * m_rem as isize;

        // 残りの候補者全員が sum_i + 1 票以上を獲得できるか
        r_rem < total_cnt - rem_total
    };
    for i in 0..n {
        let mut ng = -1;
        let mut ok = r + 1;
        while 1 < ok - ng {
            let m = (ng + ok) / 2;
            if f(i, m) {
                ok = m;
            } else {
                ng = m;
            }
        }
        if ok <= r {
            rrss[i] = ok;
        }
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
