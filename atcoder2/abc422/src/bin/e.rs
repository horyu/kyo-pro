#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables, unused_macros)]
#![feature(int_roundings)]
use itertools::{Itertools as _, chain, iproduct, izip};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

#[cfg(not(debug_assertions))]
macro_rules! eprintln {
    ($($tt:tt)*) => {};
}

// https://zenn.dev/tipstar0125/articles/245bceec86e40a
#[derive(Debug, Clone)]
struct TimeKeeper {
    start_time: std::time::Instant,
    time_threshold: f64,
}

impl TimeKeeper {
    fn new(time_threshold: f64) -> Self {
        TimeKeeper {
            start_time: std::time::Instant::now(),
            time_threshold,
        }
    }
    fn is_time_over(&self) -> bool {
        let elapsed_time = self.start_time.elapsed().as_nanos() as f64 * 1e-9;
        elapsed_time >= self.time_threshold
    }
}

fn main() {
    let time_keeper = TimeKeeper::new(1.85);
    input! {
        n: usize,
        xxyy: [(isize, isize); n],
    };
    // (x2​−x1​)(y−y1​)=(y2​−y1​)(x−x1​) を使う
    // ax+by+c=0; a=y2-y1, b=-(x2-x1), c=(x2-x1)*y1-(y2-y1)*x1
    // 要素をシャッフル
    let xxyy = HashSet::<(isize, isize)>::from_iter(xxyy)
        .into_iter()
        .collect_vec();
    for (i, (x1, y1)) in xxyy.iter().copied().enumerate() {
        if time_keeper.is_time_over() {
            break;
        }
        let mut counter = counter::Counter::<_>::new();
        for (x2, y2) in xxyy.iter().copied().skip(i + 1) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let a = dy;
            let b = -dx;
            let c = dx * y1 - dy * x1;
            let gcd = a.gcd(&b).gcd(&c);
            let div = gcd * if a != 0 { a.signum() } else { b.signum() };
            let key = [a / div, b / div, c / div];
            if key.iter().any(|k| 10isize.pow(18) < k.abs()) {
                continue;
            }
            counter[&key] += 1;
            if n / 2 <= counter[&key] {
                println!("Yes");
                println!("{}", key.iter().join(" "));
                // for (x, y) in xxyy.iter().copied() {
                //     eprintln!("{x} {y}: {}", a * x + b * y + c);
                // }
                return;
            }
        }
        // for (k, v) in counter {
        //     eprintln!("{k:?}: {v}");
        // }
    }
    println!("No");
}
