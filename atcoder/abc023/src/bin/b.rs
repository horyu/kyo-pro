#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        n: usize,
        s: String
    };
    if n % 2 == 0 {
        println!("{}", -1);
        return;
    }
    let rev_s = s.chars().rev().collect::<String>();
    let center = n / 2;
    let c_to_l = "bacbacbacbacbacbacbacbacbacbacbacbacbacbacbacbacba";
    let c_to_r = "bcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabcabc";
    if c_to_l.starts_with(&rev_s[center..]) && c_to_r.starts_with(&s[center..]) {
        println!("{}", center);
    } else {
        println!("{}", -1);
    }
}
