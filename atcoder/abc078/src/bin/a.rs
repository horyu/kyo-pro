#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::Ordering;

fn main() {
    input! {
        x: char,
        y: char
    };
    let s = match x.cmp(&y) {
        Ordering::Less => "<",
        Ordering::Equal => "=",
        Ordering::Greater => ">",
    };
    println!("{}", s);
}
