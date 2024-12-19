use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

pub struct Graph<T, V>
where
    T: Eq + Hash,
    V: Eq + Hash,
{
    nodes: HashMap<T, V>,
    edges: HashMap<T, HashSet<T>>,
}

impl<T, V> Graph<T, V>
where
    T: Eq + Hash,
    V: Eq + Hash,
{
    pub fn new() -> Self { Graph { nodes: HashMap::new(), edges: HashMap::new() } }

    pub fn add_node(&mut self, id: T, value: V) { self.nodes.insert(id, value); }

    pub fn add_edge(&mut self, from: T, to: T) {
        match self.edges.get_mut(&from) {
            None => {
                self.edges.insert(from, HashSet::from([to]));
            }
            Some(adj) => {
                adj.insert(to);
            }
        }
    }

    pub fn val(&self, id: &T) -> Option<&V> { self.nodes.get(id) }

    pub fn adj(&self, from: &T) -> Option<&HashSet<T>> { self.edges.get(from) }
}
