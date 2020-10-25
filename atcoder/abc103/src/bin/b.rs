#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    use std::collections::VecDeque;
    let s: VecDeque<_> = s.into();
    let mut t: VecDeque<_> = t.into();
    for _ in 0..s.len() {
        if s == t {
            println!("Yes");
            return;
        }
        let c = t.pop_front().unwrap();
        t.push_back(c);
    }
    println!("No");
}
