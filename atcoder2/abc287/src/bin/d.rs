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
        s: Chars,
        t: Chars,
    };
    let slen = s.len();
    let tlen = t.len();
    let mut ll = vec![false; tlen + 1];
    ll[0] = true;
    for i in 0..tlen {
        let sc = s[i];
        let tc = t[i];
        if sc == '?' || tc == '?' || sc == tc {
            ll[i + 1] = true;
        } else {
            break;
        }
    }
    let mut rr = vec![false; tlen + 1];
    rr[tlen] = true;
    for i in (0..tlen).rev() {
        let sc = s[slen - tlen + i];
        let tc = t[i];
        // eprintln!("{i} {sc} {tc}");
        if sc == '?' || tc == '?' || sc == tc {
            rr[i] = true;
        } else {
            break;
        }
    }
    for x in 0..=t.len() {
        let tf = ll[x] && rr[x];
        let rs = ["No", "Yes"][tf as usize];
        println!("{rs}");
    }
}
