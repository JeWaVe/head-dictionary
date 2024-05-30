# PrefixDictionary

A PrefixDictionary is like a dictionary, but enabling the capacity to search by prefix. It's underlying structure is a n-ary tree. 

It's generic (template) on data, but most common use is with characters. 


## Pseudo code
a = PrefixDictionary("dictionary", "lapin", "lapins", "lapine")

- a.contains("dict") -> as_prefix because "dict" is not in the word list but only a prefix
- a.contains("dictionary") -> as_word because "dictionary" is an actual word and no longer word exist
- a.contains("lapin") -> as_prefix | as_word because "lapin" is both a word and a prefix (for "lapins" and "lapine")
- a.contains("tutu") -> None because tutu is not in the dictionary

## Note 
In the examples above, only the first `t` of the word "tutu" will be read as we know (from the structure of dictionary), that no words inserted starts with the letter `t`. 


## Example

```rust
pub fn test_simple_1() {
    let mut dict = PrefixDictionary::new();
    dict.feed(&["dictionary", "lapin", "lapins", "lapine"]);
    assert_eq!(4, dict.len());

    let mut result = dict.contains("dict");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_PREFIX);

    result = dict.contains("lapin");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_PREFIX | SearchResult::AS_WORD);

    result = dict.contains("dictionary");
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_WORD);

    result = dict.contains("tutu");
    assert!(result.is_none());
}
```