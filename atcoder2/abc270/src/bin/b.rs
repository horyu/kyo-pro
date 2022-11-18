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
        mut x: isize,
        mut y: isize,
        mut z: isize,
    };
    // ゴールを右側にする
    if x < 0 {
        x = -x;
        y = -y;
        z = -z;
    }
    if x < y {
        // 間に壁がない
        println!("{x}");
    } else {
        // 壁より左にゴールがある
        if y < 0 {
            // 間に壁がない
            println!("{x}");
        } else {
            // ゴールとの間に壁がある
            if z < y {
                // 壁より左にハンマー
                let rs = if z < 0 { 2 * z.abs() + x } else { x };
                println!("{rs}");
            } else {
                // 無理
                println!("-1");
            }
        }
    }
}
