#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        d: u8
    };
    println!(
        "{}",
        match d {
            25 => "Christmas",
            24 => "Christmas Eve",
            23 => "Christmas Eve Eve",
            22 => "Christmas Eve Eve Eve",
            _ => unreachable!(),
        }
    );
}
