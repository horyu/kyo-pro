#![allow(unused_imports)]
// use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        a: String,
        b: String
    };
    if a == b {
        println!("EQUAL");
        return;
    }
    let alen = a.len();
    let blen = b.len();
    if alen > blen {
        println!("GREATER");
    } else if alen < blen {
        println!("LESS");
    } else {
        for (ac, bc) in a.chars().zip(b.chars()) {
            let acu8 = ac as u8;
            let bcu8 = bc as u8;
            if acu8 > bcu8 {
                println!("GREATER");
                return;
            } else if acu8 < bcu8 {
                println!("LESS");
                return;
            }
        }
    }
}
