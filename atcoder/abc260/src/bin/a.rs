#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]
#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::*;
use proconio::{input, marker::*};
use std::collections::*;
 
fn main() {
    input! {
        mut s: Chars
    };
    s.sort_unstable();
    if s[0] != s[1] {
        println!("{}", s[0]);
    } else if s[1] != s[2] {
        println!("{}", s[2]);
    } else {
        println!("-1");
    }
}