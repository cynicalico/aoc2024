use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Graph<T>
where
    T: Eq + Hash,
{
    nodes: HashMap<T, HashSet<T>>,
}

impl<T> Graph<T>
where
    T: Eq + Hash,
{
    pub fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        match self.nodes.get_mut(&from) {
            None => {
                self.nodes.insert(from, HashSet::from([to]));
            }
            Some(adj) => {
                adj.insert(to);
            }
        }
    }

    pub fn adj(&self, from: &T) -> Option<&HashSet<T>> {
        self.nodes.get(from)
    }
}
