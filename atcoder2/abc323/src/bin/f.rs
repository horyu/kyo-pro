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
        mut ax: i128,
        mut ay: i128,
        mut bx: i128,
        mut by: i128,
        cx: i128,
        cy: i128,
    };
    // 終点 cx, cy を原点に移動する
    ax -= cx;
    ay -= cy;
    bx -= cx;
    by -= cy;
    fn f(mut ax: i128, mut ay: i128, mut bx: i128, mut by: i128) -> i128 {
        // 第１象限に捉え直す
        if bx.is_negative() {
            ax = -ax;
            bx = -bx;
        }
        if by.is_negative() {
            ay = -ay;
            by = -by;
        }
        // y=0 なら x=0 に捉え直す
        if by == 0 {
            (ax, ay) = (ay, ax);
            (bx, by) = (by, bx);
        }
        assert!(by != 0);

        let dx = ax.abs_diff(bx) as i128;
        let dy = ay.abs_diff(by) as i128;
        use std::cmp::Ordering;
        eprintln!("{ax} {ay} {bx} {by}");
        dbg!(ax.cmp(&bx), ay.cmp(&by));
        if bx == 0 {
            /*
            |  3 4 5
            O- 1 B 2
            |  3 4 5
            */
            match (ax.cmp(&bx), ay.cmp(&by)) {
                // 1-3
                (Ordering::Equal, Ordering::Less) => 1 + f(1, ay, 0, by),
                (Ordering::Equal, Ordering::Equal) => unreachable!(),
                // 2-O
                (Ordering::Equal, Ordering::Greater) => ay - 1,
                // 3-5
                (_, Ordering::Less) => dy + 1 + f(ax, by + 1, 0, by),
                // 4-5
                (_, Ordering::Equal) => 1 + f(ax, by + 1, 0, by),
                // 5-2
                (_, Ordering::Greater) => dx + f(0, ay, 0, by),
            }
        } else {
            /*
            |  6 7 8
            9  4 B 5
            |  1 2 3
            O-
            */
            match (ax.cmp(&bx), ay.cmp(&by)) {
                // 1-3
                (Ordering::Less, Ordering::Less) => dy + 1 + f(ax, by + 1, bx, by),
                // 2-3
                (Ordering::Less, Ordering::Equal) => 1 + f(ax, by + 1, bx, by),
                // 3-5
                (Ordering::Less, Ordering::Greater) => dx + f(bx, ay, bx, by),
                // 4(2)
                (Ordering::Equal, Ordering::Less) => f(ay, ax, by, bx),
                (Ordering::Equal, Ordering::Equal) => unreachable!(),
                // 5-9
                (Ordering::Equal, Ordering::Greater) => ay - 1 + f(ax, 1, bx, 0),
                // 6(3)
                (Ordering::Greater, Ordering::Less) => f(ay, ax, by, bx),
                // 7(5)
                (Ordering::Greater, Ordering::Equal) => f(ay, ax, by, bx),
                // 8-5
                (Ordering::Greater, Ordering::Greater) => dx + f(bx, ay, bx, by),
            }
        }
    }
    let rs = f(ax, ay, bx, by);
    println!("{rs}");
}
