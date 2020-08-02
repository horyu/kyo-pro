#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        o: Chars,
        e: Chars,
    };
    let mut oi = o.iter();
    let mut ei = e.iter();
    let mut s = "".to_owned();
    while let Some(oc) = oi.next() {
        s.push(*oc);
        if let Some(ec) = ei.next() {
            s.push(*ec);
        }
    }
    println!("{}", s);
}
