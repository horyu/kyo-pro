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
        n: usize,
        m: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut yobi = BTreeSet::new();
    for (i, a) in aa[0..(m - 1)].iter().copied().enumerate() {
        yobi.insert((a, i));
    }
    let mut main = BTreeSet::new();
    let mut sum = 0;
    let mut rs = vec![];
    for (r, ar) in aa.iter().copied().enumerate().skip(m - 1) {
        yobi.insert((ar, r));
        while main.len() < k {
            if let Some((ai, i)) = yobi.pop_first() {
                sum += ai;
                main.insert((ai, i));
            }
        }

        if let Some((main_max, _)) = main.last().copied() {
            if let Some((yobi_min, _)) = yobi.first().copied() {
                if yobi_min < main_max {
                    sum = sum + yobi_min - main_max;
                    let max = main.pop_last().unwrap();
                    let min = yobi.pop_first().unwrap();
                    main.insert(min);
                    yobi.insert(max);
                }
            }
        }
        rs.push(sum);
        // eprintln!("main:{}", main.iter().map(|kv| format!("{kv:?}")).join(" "));
        // eprintln!("yobi:{}", yobi.iter().map(|kv| format!("{kv:?}")).join(" "));

        let rm_value = (aa[r + 1 - m], r + 1 - m);
        if main.remove(&rm_value) {
            sum -= rm_value.0;
        };
        yobi.remove(&rm_value);
    }
    println!("{}", rs.iter().join(" "));
}

#[allow(dead_code)]
fn main2() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        aa: [usize; n],
    };
    let mut bts1 = BTreeSet::new();
    let mut bts2 = BTreeSet::new();
    let mut sum = 0;
    for (i, a) in aa.iter().copied().enumerate().take(m) {
        bts1.insert((a, i));
        sum += a;
    }
    for _ in 0..(m - k) {
        let (a, i) = bts1.pop_last().unwrap();
        bts2.insert((a, i));
        sum -= a;
    }
    assert_eq!(bts1.len(), k);
    assert_eq!(bts1.len() + bts2.len(), m);

    let mut rrss = vec![sum];
    for r in m..n {
        let l = r - m;
        let al = aa[l];
        if bts1.remove(&(al, l)) {
            sum -= al;
            if let Some((a, i)) = bts2.pop_first() {
                bts1.insert((a, i));
                sum += a;
            }
        } else {
            bts2.remove(&(al, l));
        }
        assert_eq!(bts1.len() + bts2.len(), m - 1);
        let ar = aa[r];
        if bts1.is_empty() {
            bts1.insert((ar, r));
            sum += ar;
            rrss.push(sum);
            continue;
        }
        let last_a = bts1.last().copied().unwrap_or_default().0;
        if ar <= last_a {
            bts2.insert(bts1.pop_last().unwrap());
            sum -= last_a;
            bts1.insert((ar, r));
            sum += ar;
        } else {
            bts2.insert((ar, r));
        }
        rrss.push(sum);
        assert_eq!(bts1.len(), k);
        assert_eq!(bts1.len() + bts2.len(), m);
    }
    let rs = rrss.iter().join(" ");
    println!("{rs}");
}
