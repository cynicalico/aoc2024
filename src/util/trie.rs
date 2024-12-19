use hashbrown::HashMap;

#[derive(Default)]
struct Node {
    pub children: HashMap<char, Node>,
    pub is_terminal: bool,
}

pub struct Trie {
    root: Node,
    pub max_key_len: usize,
}

impl Trie {
    pub fn new() -> Self { Self { root: Node::default(), max_key_len: 0 } }

    pub fn insert(&mut self, key: &str) {
        let mut curr = &mut self.root;
        for ch in key.chars() {
            curr = curr.children.entry(ch).or_default();
        }
        curr.is_terminal = true;
        self.max_key_len = self.max_key_len.max(key.len());
    }

    pub fn find(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.chars() {
            match curr.children.get(&ch) {
                Some(next) => curr = next,
                None => return false,
            }
        }
        curr.is_terminal
    }
}
