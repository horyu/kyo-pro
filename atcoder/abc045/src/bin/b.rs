#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        mut sa: Chars,
        mut sb: Chars,
        mut sc: Chars
    };
    let mut c = 'a';
    loop {
        let turn = match c {
            'a' => &mut sa,
            'b' => &mut sb,
            'c' => &mut sc,
            _ => unreachable!(),
        };
        if turn.is_empty() {
            println!("{}", c.to_uppercase());
            return;
        }
        c = turn.remove(0);
    }
}
