#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        ss: Chars,
        tt: Chars
    };
    let at = "atcoder";
    for (s, t) in ss.into_iter().zip(tt) {
        if !((s == t) || ((s == '@') && at.contains(t)) || ((t == '@') && at.contains(s))) {
            println!("You will lose");
            return;
        }
    }
    println!("You can win");
}
