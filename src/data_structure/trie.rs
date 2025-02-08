
#[derive(Default, Debug, PartialEq)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end_of_word: bool,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            children: Default::default(),
            is_end_of_word: false,
        }
    }
}

#[derive(Debug)]
struct Trie {
    root: TrieNode,
}


impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn char_to_index(c: char) -> Option<usize> {
        let lowercase_c = c.to_ascii_lowercase();
        if lowercase_c.is_ascii_alphabetic() {
            Some((lowercase_c as usize) - ('a' as usize))
        } else {
            None
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            if let Some(index) = Trie::char_to_index(c) {
                node = node.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
            }
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(index) = Trie::char_to_index(c) {
                match &node.children[index] {
                    Some(child) => node = child,
                    None => return false,
                }
            } else {
                return false;
            }
        }
        node.is_end_of_word
    }
    
    fn get(&self, word: &str) -> Option<Vec<String>> {
        let mut node = &self.root;
        for c in word.chars() {
            if let Some(index) = Trie::char_to_index(c) {
                match &node.children[index] {
                    Some(child) => node = child,
                    None => return None,
                }
            } else {
                return None;
            }
        }
        
        let mut words: Vec<String> = Vec::new();
        let mut stack: Vec<(&TrieNode, String)> = Vec::new();
        stack.push((node, word.to_ascii_lowercase()));
        
        while let Some((n, curr_str)) = stack.pop() {
            if n.is_end_of_word {
                words.push(curr_str.clone());
            }  
            for (idx, child) in n.children.iter().enumerate() {
                if let Some(c) = child {
                    let found_char = (idx as u8 + b'a') as char;
                    let mut new_str = curr_str.clone();
                    new_str.push(found_char);
                    stack.push((c, new_str));
                }
            }
        }
        Some(words)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn init_trienode() {
        let tnode = TrieNode::new();
        assert_eq!(tnode, TrieNode {
            children: [const{None}; 26],
            is_end_of_word: false,
        });
    }

    #[test]
    fn init_trie() {
        let trie = Trie::new();
        assert_eq!(trie.root, TrieNode::new());
    }

    #[test]
    fn insert_and_search_single_word() {
        let mut trie = Trie::new();
        trie.insert("hello");

        assert!(trie.search("hello")); // Should return true
        assert!(!trie.search("hell")); // Should return false
        assert!(!trie.search("world")); // Should return false
    }

    #[test]
    fn insert_and_search_multiple_words() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        trie.insert("apex");

        assert!(trie.search("apple"));
        assert!(trie.search("app"));
        assert!(trie.search("apex"));

        assert!(!trie.search("apples"));
        assert!(!trie.search("ape"));
    }

    #[test]
    fn search_empty_trie() {
        let trie = Trie::new();
        assert!(!trie.search("test"));
    }

    #[test]
    fn get_words_with_prefix() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("app");
        trie.insert("application");
        trie.insert("apply");
        trie.insert("apt");

        let words = trie.get("app").unwrap();
        let mut expected_words = vec!["app".to_string(), "apple".to_string(), "application".to_string(), "apply".to_string()];
        expected_words.sort();
        let mut retrieved_words = words.clone();
        retrieved_words.sort();

        assert_eq!(retrieved_words, expected_words);

        let words2 = trie.get("apt").unwrap();
        assert_eq!(words2, vec!["apt".to_string()]);

        let words3 = trie.get("xyz");
        assert!(words3.is_none());
    }

    #[test]
    fn insert_duplicate_words() {
        let mut trie = Trie::new();
        trie.insert("test");
        trie.insert("test");

        assert!(trie.search("test"));
    }

    #[test]
    fn insert_and_search_case_insensitive() {
        let mut trie = Trie::new();
        trie.insert("Hello");

        assert!(trie.search("hello")); // Should return true
        assert!(trie.search("HELLO")); // Should return true
        assert!(trie.search("HeLLo")); // Should return true
        assert!(!trie.search("Helloo")); // Should return false
    }

    #[test]
    fn get_with_case_insensitivity() {
        let mut trie = Trie::new();
        trie.insert("App");
        trie.insert("Apple");
        trie.insert("Application");

        let words = trie.get("APP").unwrap();
        let mut expected_words = vec!["app".to_string(), "apple".to_string(), "application".to_string()];
        expected_words.sort();
        let mut retrieved_words = words.clone();
        retrieved_words.sort();

        assert_eq!(retrieved_words, expected_words);
    }

    #[test]
    fn insert_with_mixed_case() {
        let mut trie = Trie::new();
        trie.insert("TeSt");

        assert!(trie.search("test")); // Should return true
        assert!(trie.search("TEST")); // Should return true
        assert!(trie.search("TeSt")); // Should return true
    }
}