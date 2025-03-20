use std::collections::{HashMap, HashSet};

use super::weighted_graph;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Str(String),
    Int(i32),
}

struct Graph {
    nodes: HashSet<Node>,
    neighbors: HashMap<Node, HashSet<Node>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: HashSet::new(),
            neighbors: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.clone());
        self.neighbors.entry(node).or_insert(HashSet::new());
    }

    pub fn add_edge(&mut self, node1: Node, node2: Node) {
        self.neighbors
            .entry(node1.clone())
            .or_insert(HashSet::new())
            .insert(node2.clone());
    }

    pub fn remove_node(&mut self, node: Node) {
        self.nodes.remove(&node);
        self.neighbors.remove(&node);
        for (_, neighbors) in self.neighbors.iter_mut() {
            neighbors.remove(&node);
        }
    }

    pub fn remove_edge(&mut self, node1: Node, node2: Node) {
        if let Some(neighbors) = self.neighbors.get_mut(&node1) {
            neighbors.remove(&node2);
        }
        if let Some(neighbors) = self.neighbors.get_mut(&node2) {
            neighbors.remove(&node1);
        }
    }

    pub fn get_neighbors(&self, node: Node) -> Option<&HashSet<Node>> {
        self.neighbors.get(&node)
    }

    pub fn create_from_edges(edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Graph::new();
        for edge in edges {
            let node1 = Node::Int(edge[0]);
            let node2 = Node::Int(edge[1]);
            graph.add_node(node1.clone());
            graph.add_node(node2.clone());
            graph.add_edge(node1, node2);
        }
        graph
    }

    pub fn create_from_edges_undirected(edges: Vec<Vec<i32>>) -> Self {
        let mut graph = Graph::new();
        for edge in edges {
            let node1 = Node::Int(edge[0]);
            let node2 = Node::Int(edge[1]);
            graph.add_node(node1.clone());
            graph.add_node(node2.clone());
            graph.add_edge(node1.clone(), node2.clone());
            graph.add_edge(node2.clone(), node1.clone());
        }
        graph
    }

    pub fn create_from_adjacency_list(edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let mut graph = HashMap::new();
        for edge in edges {
            let node1 = edge[0];
            let node2 = edge[1];
            graph.entry(node1).or_insert(Vec::new()).push(node2);
            graph.entry(node2).or_insert(Vec::new()).push(node1);
        }
        graph
    }

    pub fn create_from_adjacency_list_adjacency_list(
        edges: Vec<Vec<i32>>,
    ) -> HashMap<i32, Vec<i32>> {
        let mut graph = HashMap::new();
        for edge in edges {
            let node1 = edge[0];
            let node2 = edge[1];
            graph.entry(node1).or_insert(Vec::new()).push(node2);
        }
        graph
    }
}
