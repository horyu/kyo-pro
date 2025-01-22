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
f(2468) = (4*8) + (3*6+1*60) + (2*4+2*40+2*400) + (1*2+1*20+1*200+1*2000)

(2*1) + ((2*1)*10 + (4*2)) + ((2*10 + (4*2))*10 + (6*3))
S(1~1) =                        1 * x[1];
S(1~2) = S(1~1) + S(1~1) * 10 + 2 * x[2];
S(1~3) = S(1~2) + S(1~2) * 10 + 3 * x[3];

*/

fn main() {
    input! {
        n: usize,
        s: Bytes,
    };
    let bb = s.iter().map(|b| (b - b'0') as usize).collect_vec();
    let iibb = izip!(1usize.., bb.iter().copied()).collect_vec();
    let mut sum = iibb.iter().copied().fold(0, |acc, (i, b)| {
        acc + i * b
    });
    let mut vv = vec![];
    for (i, b) in iibb.iter().copied().rev() {
        vv.push(sum);
        sum -= i * b;
    }
    let mut qq = VecDeque::new();
    let mut tmp = 0;
    for v in vv {
        let (div, r) = (v + tmp).div_rem(&10);
        qq.push_front(r);
        tmp = div;
    }
    while 0 < tmp {
        qq.push_front(tmp % 10);
        tmp /= 10;
    }
    let rs = qq.iter().join("");
    println!("{rs}" );
}
