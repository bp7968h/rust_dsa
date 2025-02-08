
#[derive(Default, Debug)]
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

    fn char_to_index(c: char) -> usize {
        (c as usize) - ('a' as usize)
    }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            let index = Trie::char_to_index(c);
            node = node.children[index].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            let index = Trie::char_to_index(c);
            match &node.children[index] {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end_of_word
    }
    
    //#[derive(Default, Debug)]
    //struct TrieNode {
    //    children: [Option<Box<TrieNode>>; 26],
    //    is_end_of_word: bool,
    //}
    
    fn get(&self, word: &str) -> Option<Vec<String>> {
        let mut node = &self.root;
        for c in word.chars() {
            let index = Trie::char_to_index(c);
            match &node.children[index] {
                Some(child) => node = child,
                None => return None,
            }
        }
        
        let mut words: Vec<String> = Vec::new();
        let mut stack: Vec<(&TrieNode, String)> = Vec::new();
        stack.push((node, word.to_string()));
        
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