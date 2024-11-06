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
        d: usize,
        xxyy: [(isize, isize); n],
    };
    let xxyy = xxyy
        .into_iter()
        .map(|(x, y)| ((x + 2e6 as isize) as usize, (y + 2e6 as isize) as usize))
        .collect_vec();

    // 軸ごとの出現回数
    let mut ftx = ac_library::FenwickTree::new(3e6 as usize + 1, 0usize);
    let mut fty = ac_library::FenwickTree::new(3e6 as usize + 1, 0usize);
    for (x, y) in xxyy.iter().copied() {
        ftx.add(x, 1);
        fty.add(y, 1);
    }
    // 重心
    let (mut gx, mut gy) = (0, 0);
    for (x, y) in xxyy.iter().copied() {
        gx += x;
        gy += y;
    }
    gx /= n;
    gy /= n;
    // dbg!(gx, gy);
    // 重心のコスト
    let (mut cgx, mut cgy) = (0, 0);
    for (x, y) in xxyy.iter().copied() {
        cgx += gx.abs_diff(x);
        cgy += gy.abs_diff(y);
    }
    if d < cgx + cgy {
        println!("0");
        return;
    }

    // min_memo[x] = 条件を満たす(y_min, cost_y)
    // max_memo[x] = 条件を満たす(y_max, cost_y)
    let mut min_memo = vec![(0, 0); 3e6 as usize + 1];
    let mut max_memo = vec![(0, 0); 3e6 as usize + 1];
    // cx における y_min, y_max を求める
    let mut cost_y = cgy;
    for y in (0..gy).rev() {
        let new_cost = cost_y - fty.sum(..=y) + fty.sum((y + 1)..);
        if d < cgx + new_cost {
            min_memo[gx] = (y + 1, cost_y);
            break;
        }
        cost_y = new_cost;
    }
    let mut cost_y = cgy;
    for y in (gy + 1).. {
        let new_cost = cost_y + fty.sum(..y) - fty.sum(y..);
        if d < cgx + new_cost {
            max_memo[gx] = (y - 1, cost_y);
            break;
        }
        cost_y = new_cost;
    }

    // gx から左右の伸ばしていく際、直前の値を利用する
    let mut rs = max_memo[gx].0 - min_memo[gx].0 + 1;
    let mut cost_x = cgx;
    for x in (0..gx).rev() {
        cost_x = cost_x - ftx.sum(..=x) + ftx.sum((x + 1)..);
        if d < cost_x + cgy {
            break;
        }
        let pre_x = x + 1;

        let mut cost_y = min_memo[pre_x].1;
        for y in min_memo[pre_x].0..=gy {
            if cost_x + cost_y <= d {
                min_memo[x] = (y, cost_y);
                break;
            }
            let next_y = y + 1;
            cost_y = cost_y + fty.sum(..next_y) - fty.sum(next_y..);
        }
        let mut cost_y = max_memo[pre_x].1;
        for y in (gy..=max_memo[pre_x].0).rev() {
            if cost_x + cost_y <= d {
                max_memo[x] = (y, cost_y);
                break;
            }
            let next_y = y - 1;
            cost_y = cost_y - fty.sum(..=next_y) + fty.sum((next_y + 1)..);
        }
        rs += max_memo[x].0 - min_memo[x].0 + 1;
    }
    let mut cost_x = cgx;
    for x in (gx + 1).. {
        cost_x = cost_x + ftx.sum(..x) - ftx.sum(x..);
        if d < cost_x + cgy {
            break;
        }
        let pre_x = x - 1;

        let mut cost_y = min_memo[pre_x].1;
        for y in min_memo[pre_x].0..=gy {
            if cost_x + cost_y <= d {
                min_memo[x] = (y, cost_y);
                break;
            }
            let next_y = y + 1;
            cost_y = cost_y + fty.sum(..next_y) - fty.sum(next_y..);
        }
        let mut cost_y = max_memo[pre_x].1;
        for y in (gy..=max_memo[pre_x].0).rev() {
            if cost_x + cost_y <= d {
                max_memo[x] = (y, cost_y);
                break;
            }
            let next_y = y - 1;
            cost_y = cost_y - fty.sum(..=next_y) + fty.sum((next_y + 1)..);
        }
        rs += max_memo[x].0 - min_memo[x].0 + 1;
    }

    println!("{rs}");
}
