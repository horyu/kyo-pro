#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars
    };
    // 尺取法　https://blog.hamayanhamayan.com/entry/2019/04/14/100439
    let blocks = s
        .into_iter()
        .group_by(|c| *c == '0')
        .into_iter()
        .map(|(c, block)| (c, block.count()))
        .collect_vec();
    let m = blocks.len();
    let mut rs = 0;
    let mut sm = 0;
    let mut l = 0;
    let mut zeros = 0;
    for r in 0..m {
        sm += blocks[r].1;
        if blocks[r].0 {
            zeros += 1;
        }
        while k < zeros {
            sm -= blocks[l].1;
            if blocks[l].0 {
                zeros -= 1;
            }
            l += 1;
        }

        rs = rs.max(sm);
    }
    println!("{}", rs);
}

// 圧縮＋累積和
fn main2() {
    input! {
        _n: usize,
        k: usize,
        s: Chars
    };
    // ある点から全部右連結させていく最大と左連結させていく最大を計算
    let mut ttff = vec![];
    let mut cc = vec![];
    for (tf, group) in s.into_iter().group_by(|c| *c == '1').into_iter() {
        ttff.push(tf);
        cc.push(group.count());
    }
    let mut rs = 1;
    let len = cc.len();
    let sums = cc.into_iter().fold(vec![0], |mut v, c| {
        v.push(v.last().unwrap() + c);
        v
    });
    for i in 0..len {
        let (left, right) = if ttff[i] {
            (i.saturating_sub(2 * k), (i + 2 * k).min(len))
        } else {
            (i.saturating_sub(2 * k - 1), (i + 2 * k - 1).min(len))
        };
        rs = rs.max(sums[i + 1] - sums[left]);
        rs = rs.max(sums[right] - sums[i + 1]);
    }
    println!("{}", rs);
}
