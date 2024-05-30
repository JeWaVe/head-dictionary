use std::collections::HashMap;
use std::marker::PhantomData;
use core::hash::Hash;

bitflags::bitflags! {
    /// Enum used for [PrefixDictionary::contains] method
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct SearchResult: u8 {
        const _ = 0x00;
        /// the "word" exists in dictionary
        const AS_WORD = 0x01;
        /// the "word" exists as prefix (longer words starting with it exist)
        const AS_PREFIX = 0x02;
    }
}

struct Node<I> {
    is_end_of_word: bool,
    childs:  HashMap<I, Node<I>>
}

impl<I> Node<I> {
    pub fn new() -> Self {
        Node {
            is_end_of_word : false,
            childs : HashMap::new()
        }
    }
}

/// main structure
pub struct PrefixDictionary<I, T> {
    root: Node<I>,
    count: u64,
    _phantom_i: PhantomData<I>,
    _phantom_t: PhantomData<T>
}

impl<I, T>  PrefixDictionary<I, T> 
    where T: IntoIterator<Item = I>,
          I: Eq,
          I: PartialEq<I>,
          I: Hash,
          I: Copy {

    /// base constructor
    pub fn new () -> Self {
        PrefixDictionary {
            root: Node::new(),
            count: 0,
            _phantom_i: PhantomData,
            _phantom_t: PhantomData
        }
    }

    /// returns element count
    pub fn len(&self) -> u64 {
        self.count
    }

    /// inserts (see [insert]) all words in dictionary
    pub fn feed(&mut self, words: impl IntoIterator<Item = T>) {
        for w in words {
            self.insert(w);
        }
    }

    /// inserts `word` in the dictionary
    pub fn insert(&mut self, word: T) {
        let mut node = &mut self.root;
        for c in word.into_iter() {
            if !node.childs.contains_key(&c) {
                node.childs.insert(c, Node::new());
            }

            node = node.childs.get_mut(&c).unwrap();
        }

        node.is_end_of_word = true;
        self.count += 1
    }

    /// search in the dictionary
    /// returns none if the word is not inside dictionary
    /// or a combination of [SearchResult::AS_PREFIX] and [SearchResult::AS_WORD] if it is inside
    pub fn contains(&mut self, word: T) -> Option<SearchResult> {
        let mut node = &mut self.root;
        for c in word.into_iter() {
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
