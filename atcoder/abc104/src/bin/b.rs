#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    if (s[0] != 'A')
        || (s[2..(s.len() - 1)].iter().filter(|&&c| c == 'C').count() != 1)
        || (s.iter().filter(|&&c| c.is_ascii_uppercase()).count() != 2)
    {
        println!("WA");
        return;
    }
    println!("AC");
}
