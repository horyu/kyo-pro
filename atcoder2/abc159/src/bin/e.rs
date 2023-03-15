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
        h: usize,
        w: usize,
        k: usize,
        s: [Chars; h],
    };
    // https://blog.hamayanhamayan.com/entry/2020/03/22/231629
    let mut rs = !0usize;
    'outer: for div in 0..(1 << (h - 1)) {
        // グループ分け
        let mut gg = vec![];
        let mut g = 0;
        let mut cnt = 0;
        for i in 0..(h - 1) {
            gg.push(g);
            if 0 < (div & (1 << i)) {
                g += 1;
                cnt += 1;
            }
        }
        gg.push(g);
        // eprintln!("{}", gg.iter().join(" "));

        // 計算
        let mut x = 0;
        while x < w {
            let mut cc = vec![0; gg.len()];
            let mut nxt = x;
            for xx in x..w {
                let mut ng = false;
                for y in 0..h {
                    if s[y][xx] == '1' {
                        cc[gg[y]] += 1;
                        if k < cc[gg[y]] {
                            ng = true;
                            break;
                        }
                    }
                }
                if ng {
                    break;
                }
                nxt = nxt.max(xx + 1);
            }
            if nxt == x {
                continue 'outer;
            }
            x = nxt;
            cnt += 1;
        }
        rs = rs.min(cnt - 1);
    }
    println!("{rs}");
}
