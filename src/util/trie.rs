use hashbrown::HashMap;

struct Node {
    pub children: HashMap<u8, Box<Node>>,
    pub is_terminal: bool,
}

impl Node {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_terminal: false,
        }
    }
}

pub struct Trie {
    root: Box<Node>,
    pub max_key_len: usize,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
            max_key_len: 0,
        }
    }

    pub fn insert(&mut self, key: &str) {
        let mut curr = &mut self.root;
        for ch in key.as_bytes() {
            curr = curr.children.entry(*ch).or_insert(Box::new(Node::new()));
        }
        curr.is_terminal = true;
        self.max_key_len = self.max_key_len.max(key.len());
    }

    pub fn find(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.as_bytes() {
            match curr.children.get(ch) {
                None => return false,
                Some(next) => curr = next,
            }
        }
        true
    }

    pub fn find_terminal(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.as_bytes() {
            match curr.children.get(ch) {
                None => return false,
                Some(next) => curr = next,
            }
        }
        curr.is_terminal
    }
}
