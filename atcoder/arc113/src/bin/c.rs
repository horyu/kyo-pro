#![allow(clippy::many_single_char_names)]
#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars,
    };
    let mut vv = vec![('!', 0); s.len()];
    {
        let mut r = 0;
        for (c, g) in s.iter().group_by(|&&c| c).into_iter() {
            let count = g.into_iter().count();
            vv[r] = (c, count);
            r += count;
        }
    }
    let mut rs = 0;
    for r in (0..s.len()).rev() {
        if vv[r].0 == '!' || vv[r].1 < 2 {
            continue;
        }
        let mut rr = r + vv[r].1;
        while rr < s.len() {
            if vv[r].0 != vv[rr].0 {
                rs += vv[rr].1;
            }
            vv[r].1 += vv[rr].1;
            rr += vv[rr].1;
        }
    }
    println!("{rs}");
}
