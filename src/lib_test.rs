use crate::{PrefixDictionary, SearchResult};

#[test]
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