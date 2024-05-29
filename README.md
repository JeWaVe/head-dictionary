# head-dictionary

A HeadDictionary is like a dictionary, but enabling the capacity to search by prefix in O(1). 

It's generic (template) on data, but most common use is with characters. 

a = HeadDictionary<char>("dictionary", "lapin", "lapins", "lapine")
a.contains("dict") -> as_prefix because "dict" is not in the word list but only a prefix
a.contains("dictionary") -> as_word because "dictionary" is an actual word and no longer word exist
a.contains("lapin") -> as_prefix | as_word because "lapin" is both a word and a prefix (for "lapins" and "lapine")
