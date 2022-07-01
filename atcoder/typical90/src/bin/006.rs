#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        cc: Chars,
    };
    // 0..(n-(k-1))までの辞書順最小を見つけ
    // (i+1)..(n-(k-2))までの辞書順最小を見つけ
    // (j+1)..(n-(k-3))までの辞書順最小を見つけ
    let mut btm = BTreeMap::new();
    for (i, &c) in cc[..(n - k)].iter().enumerate() {
        btm.entry(c).or_insert_with(VecDeque::new).push_back(i);
    }
    let mut rs = String::new();
    let mut from = 0;
    for r in (n - k)..n {
        btm.entry(cc[r]).or_default().push_back(r);
        let mut to = 0;
        if let Some((&c, vv)) = btm.iter_mut().next() {
            rs.push(c);
            to = vv[0];
        }
        for &c in &cc[from..=to] {
            if let Some(vv) = btm.get_mut(&c) {
                vv.pop_front();
                if vv.is_empty() {
                    btm.remove(&c);
                }
            }
        }
        from = to + 1;
    }
    println!("{rs}");
}
