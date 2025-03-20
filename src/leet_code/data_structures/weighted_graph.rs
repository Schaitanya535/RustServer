use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Node {
    Int(i32),
    Str(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct WeightedNode {
    node: Node,
    weight: i32,
}

pub struct WeightedGraph {
    edges: HashMap<WeightedNode, HashSet<WeightedNode>>,
}

impl WeightedGraph {
    pub fn new() -> Self {
        WeightedGraph {
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.edges
            .entry(WeightedNode { node, weight: 0 })
            .or_insert(HashSet::new());
    }

    pub fn add_edge(&mut self, node1: Node, node2: Node, weight: i32) {
        self.edges
            .entry(WeightedNode {
                node: node1,
                weight,
            })
            .or_insert(HashSet::new())
            .insert(WeightedNode {
                node: node2,
                weight,
            });
    }

    pub fn remove_node(&mut self, node: Node) {
        self.edges.remove(&WeightedNode { node, weight: 0 });
    }

    pub fn remove_edge(&mut self, node1: Node, node2: Node, weight: i32) {
        if let Some(edges) = self.edges.get_mut(&WeightedNode {
            node: node1,
            weight,
        }) {
            edges.remove(&WeightedNode {
                node: node2,
                weight,
            });
        }
    }

    pub fn get_neighbors(&self, node: Node) -> Option<&HashSet<WeightedNode>> {
        self.edges.get(&WeightedNode { node, weight: 0 })
    }

    pub fn create_from_weighted_matrix(matrix: Vec<Vec<i32>>) -> Self {
        let mut graph = WeightedGraph::new();
        for (i, row) in matrix.iter().enumerate() {
            for (j, &weight) in row.iter().enumerate() {
                if weight != 0 {
                    graph.add_edge(Node::Int(i as i32), Node::Int(j as i32), weight);
                }
            }
        }
        graph
    }
}
