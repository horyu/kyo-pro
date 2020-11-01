#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};

fn main() {
    input! {
        s: Chars
    };
    let len = s.len();
    let tf = if len <= 3 {
        s.iter().permutations(len).any(|v| {
            let num: usize = v.into_iter().collect::<String>().parse().unwrap();
            num % 8 == 0
        })
    } else {
        // puts (0..999).filter{|i| i%8 == 0}.map{|i| format('"%03d"', i)}.reject{|s| s.include?("0")}.join(", ")
        let arr = [
            "112", "128", "136", "144", "152", "168", "176", "184", "192", "216", "224", "232",
            "248", "256", "264", "272", "288", "296", "312", "328", "336", "344", "352", "368",
            "376", "384", "392", "416", "424", "432", "448", "456", "464", "472", "488", "496",
            "512", "528", "536", "544", "552", "568", "576", "584", "592", "616", "624", "632",
            "648", "656", "664", "672", "688", "696", "712", "728", "736", "744", "752", "768",
            "776", "784", "792", "816", "824", "832", "848", "856", "864", "872", "888", "896",
            "912", "928", "936", "944", "952", "968", "976", "984", "992",
        ];
        let mut hm = std::collections::HashMap::new();
        for c in s {
            *hm.entry(c).or_insert(0) += 1;
        }
        arr.iter().any(|&sanketa| {
            let mut sanketa_hm = std::collections::HashMap::new();
            for c in sanketa.chars() {
                *sanketa_hm.entry(c).or_insert(0) += 1;
            }
            sanketa_hm
                .iter()
                .all(|(&c, &count)| *hm.entry(c).or_default() >= count)
        })
    };
    println!("{}", ["No", "Yes"][tf as usize]);
}
