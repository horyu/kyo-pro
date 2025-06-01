#![allow(clippy::many_single_char_names, clippy::needless_range_loop, clippy::collapsible_else_if)]
#![allow(unused_imports, unused_variables)]
#![feature(int_roundings)]
use ac_library::ModInt998244353;
use itertools::{chain, iproduct, izip, Itertools as _};
use itertools_num::ItertoolsNum as _;
use num_integer::*;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::*};
use std::cmp::{Ordering, Reverse as R};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    input! {
        q: usize,
        ttss: [(usize, Chars); q],
    };
    let mut xx = Trie::<26, 'a'>::new();
    let mut yy = xx.clone();
    for (t, s) in ttss {
        if t == 1 {
            yy.remove(&s, true);
            xx.insert(&s);
        } else {
            if !xx.has_registered_prefix(&s) {
                yy.insert(&s);
            }
        }
        let rs = yy.count();
        println!("{rs}");
    }
}

// https://algo-logic.info/trie-tree/
#[allow(unused)]
#[derive(Clone, Debug)]
struct Node<const CHAR_SIZE: usize> {
    next: Vec<Option<usize>>,
    accept: Vec<usize>,
    c: usize,
    common: usize,
}

impl<const CHAR_SIZE: usize> Node<CHAR_SIZE> {
    fn new(c: usize) -> Self {
        Self {
            next: vec![None; CHAR_SIZE],
            accept: vec![],
            c,
            common: 0,
        }
    }
}

#[derive(Clone, Debug)]
struct Trie<const CHAR_SIZE: usize, const BASE: char> {
    nodes: Vec<Node<CHAR_SIZE>>,
    node_id: usize,
    word_id: usize,
}

#[allow(unused)]
impl<const CHAR_SIZE: usize, const BASE: char> Trie<CHAR_SIZE, BASE> {
    fn new() -> Self {
        let nodes = vec![Node::new(0)];
        Self {
            nodes,
            node_id: 1,
            word_id: 0,
        }
    }

    // Insert a word and return its id
    fn insert(&mut self, word: &[char]) -> usize {
        let mut node_id = 0;
        for &ch in word {
            let c = (ch as u8 - BASE as u8) as usize;
            if self.nodes[node_id].next[c].is_none() {
                self.nodes[node_id].next[c] = Some(self.node_id);
                self.nodes.push(Node::new(c));
                self.node_id += 1;
            }
            self.nodes[node_id].common += 1;
            node_id = self.nodes[node_id].next[c].unwrap();
        }
        self.nodes[node_id].common += 1;
        let current_word_id = self.word_id;
        self.nodes[node_id].accept.push(current_word_id);
        self.word_id += 1;
        current_word_id
    }

    // Return removed word_ids
    fn remove(&mut self, word: &[char], prefix: bool) -> Vec<usize> {
        let mut node_id = 0;
        for &ch in word {
            let c = (ch as u8 - BASE as u8) as usize;
            match self.nodes[node_id].next[c] {
                Some(next_id) => node_id = next_id,
                None => return vec![],
            }
        }
        if !prefix {
            let word_ids = std::mem::take(&mut self.nodes[node_id].accept);
            let remove_count = word_ids.len();
            {
                let mut tmp_node_id = 0;
                for &ch in word {
                    let c = (ch as u8 - BASE as u8) as usize;
                    self.nodes[tmp_node_id].common -= remove_count;
                    if self.nodes[tmp_node_id].common == 0 {
                        self.nodes[tmp_node_id].next[c] = None;
                        break;
                    }
                    tmp_node_id = self.nodes[tmp_node_id].next[c].unwrap();
                }
                self.nodes[node_id].common -= remove_count;
            }
            return word_ids;
        }
        let remove_count = self.nodes[node_id].common;
        if remove_count == 0 {
            return vec![];
        }
        let mut word_ids = vec![];
        let mut qq = VecDeque::new();
        qq.push_back(node_id);
        while let Some(current_id) = qq.pop_front() {
            word_ids.append(&mut self.nodes[current_id].accept);
            for &next_id in self.nodes[current_id].next.iter().flatten() {
                qq.push_back(next_id);
            }
        }
        {
            let mut tmp_node_id = 0;
            for &ch in word {
                let c = (ch as u8 - BASE as u8) as usize;
                self.nodes[tmp_node_id].common -= remove_count;
                if self.nodes[tmp_node_id].common == 0 {
                    self.nodes[tmp_node_id].next[c] = None;
                    break;
                }
                tmp_node_id = self.nodes[tmp_node_id].next[c].unwrap();
            }
            self.nodes[node_id].common -= remove_count;
        }
        word_ids
    }

    // Search for a word or prefix
    fn search(&self, word: &[char], prefix: bool) -> bool {
        let mut node_id = 0;
        for &ch in word {
            let c = (ch as u8 - BASE as u8) as usize;
            match self.nodes[node_id].next[c] {
                Some(next_id) => node_id = next_id,
                None => return false,
            }
        }
        if prefix {
            true
        } else {
            !self.nodes[node_id].accept.is_empty()
        }
    }

    // Check if the trie contains a word
    fn start_with(&self, prefix: &[char]) -> bool {
        self.search(prefix, true)
    }

    // Check if the trie contains an exact word
    fn contains(&self, word: &[char]) -> bool {
        self.search(word, false)
    }

    // Check if any prefix of `word` is registered as a word in the trie
    fn has_registered_prefix(&self, word: &[char]) -> bool {
        let mut node_id = 0;
        for &ch in word {
            if !self.nodes[node_id].accept.is_empty() {
                return true;
            }
            let c = (ch as u8 - BASE as u8) as usize;
            match self.nodes[node_id].next[c] {
                Some(next_id) => node_id = next_id,
                None => return false,
            }
        }
        !self.nodes[node_id].accept.is_empty()
    }

    // Return the number of words in the trie
    fn count(&self) -> usize {
        self.nodes[0].common
    }
}
