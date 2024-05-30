use std::collections::HashMap;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SearchResult: u8 {
        const _ = 0x00;
        const AS_WORD = 0x01;
        const AS_PREFIX = 0x02;
    }
}

struct Node {
    is_end_of_word: bool,
    childs:  HashMap<char, Node>
}

impl Node {
    pub fn new() -> Self {
        Node {
            is_end_of_word : false,
            childs : HashMap::new()
        }
    }
}

pub struct PrefixDictionary {
    root: Node,
    count: u64
}

impl PrefixDictionary {
    pub fn new () -> Self {
        PrefixDictionary {
            root: Node::new(),
            count: 0
        }
    }

    pub fn len(&self) -> u64 {
        self.count
    }

    pub fn feed(&mut self, words: &[&str]) {
        for w in words {
            self.insert(w);
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            if !node.childs.contains_key(&c) {
                node.childs.insert(c, Node::new());
            }

            node = node.childs.get_mut(&c).unwrap();
        }

        node.is_end_of_word = true;
        self.count += 1
    }

    pub fn contains(&mut self, word: &str) -> Option<SearchResult> {
        let mut node = &mut self.root;
        for c in word.chars() {
            if !node.childs.contains_key(&c) {
                return None;
            }
            else {
                node = node.childs.get_mut(&c).unwrap();
            }
        }

        let mut result = SearchResult::empty();
        if node.is_end_of_word {
            result |= SearchResult::AS_WORD;
        }
        if node.childs.len() > 0 {
            result |= SearchResult::AS_PREFIX;
        }

        Some(result)
    }
}


#[cfg(test)]
pub mod lib_test;
