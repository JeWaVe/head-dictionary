use crate::{PrefixDictionary, SearchResult};

#[test]
pub fn test_simple_1() {
    let mut dict = PrefixDictionary::new();
    dict.feed(["dictionary", "lapin", "lapins", "lapine"].map(|x| x.chars()));
    assert_eq!(4, dict.len());

    let mut result = dict.contains("dict".chars());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_PREFIX);

    result = dict.contains("lapin".chars());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_PREFIX | SearchResult::AS_WORD);

    result = dict.contains("dictionary".chars());
    assert!(result.is_some());
    assert_eq!(result.unwrap(), SearchResult::AS_WORD);

    result = dict.contains("tutu".chars());
    assert!(result.is_none());
}