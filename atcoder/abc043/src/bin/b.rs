#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let mut vec = vec![];
    for c in s {
        match c {
            '0' | '1' => vec.push(c),
            _ if !vec.is_empty() => {
                vec.pop().unwrap();
            }
            _ => (),
        }
    }
    println!("{}", vec.iter().collect::<String>());
}
