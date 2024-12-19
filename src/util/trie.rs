use hashbrown::HashMap;

struct Node {
    pub children: HashMap<char, Box<Node>>,
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
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: Box::new(Node::new()),
        }
    }

    pub fn insert(&mut self, key: &str) {
        let mut curr = &mut self.root;
        for ch in key.chars() {
            if !curr.children.contains_key(&ch) {
                curr.children.insert(ch, Box::new(Node::new()));
            }
            curr = curr.children.get_mut(&ch).unwrap();
        }
        curr.is_terminal = true;
    }

    pub fn find(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.chars() {
            match curr.children.get(&ch) {
                None => return false,
                Some(next) => curr = next,
            }
        }
        true
    }

    pub fn find_terminal(&self, key: &str) -> bool {
        let mut curr = &self.root;
        for ch in key.chars() {
            match curr.children.get(&ch) {
                None => return false,
                Some(next) => curr = next,
            }
        }
        curr.is_terminal
    }
}
