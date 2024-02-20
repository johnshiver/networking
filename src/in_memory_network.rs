extern crate petgraph;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

pub struct InMemoryNetwork {
    graph: UnGraph<String, ()>, // Undirected graph with String as Node data and unit type for edges
    node_indices: HashMap<String, NodeIndex>, // Mapping of node data to their NodeIndex for quick access
}

impl InMemoryNetwork {
    pub fn new() -> Self {
        InMemoryNetwork {
            graph: UnGraph::new_undirected(),
            node_indices: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: &str) {
        let node_index = self.graph.add_node(name.to_string());
        self.node_indices.insert(name.to_string(), node_index);
    }

    // Clears all connections in the graph
    pub fn clear_connections(&mut self) {
        let edge_ids: Vec<_> = self.graph.edge_indices().collect();
        for edge_id in edge_ids {
            self.graph.remove_edge(edge_id);
        }
    }

    // Partition nodes and fully connect within each group
    pub fn fully_connect_groups(&mut self, group_size: usize) {
        self.clear_connections(); // Clear existing edges first

        let nodes: Vec<_> = self.graph.node_indices().collect();
        let node_chunks: Vec<_> = nodes.chunks(group_size).collect();

        for chunk in node_chunks {
            for &node_a in chunk.iter() {
                for &node_b in chunk.iter() {
                    if node_a != node_b && !self.graph.contains_edge(node_a, node_b) {
                        self.graph.add_edge(node_a, node_b, ());
                    }
                }
            }
        }
    }

    // Method to check if two nodes are connected
    pub fn are_connected(&self, node1: &str, node2: &str) -> bool {
        let node_index1 = self.node_indices.get(node1);
        let node_index2 = self.node_indices.get(node2);

        match (node_index1, node_index2) {
            (Some(&index1), Some(&index2)) => self.graph.find_edge(index1, index2).is_some(),
            _ => false, // One or both nodes do not exist in the graph
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_nodes() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");

        assert_eq!(graph.graph.node_count(), 2);
        assert!(graph.node_indices.contains_key("Node1"));
        assert!(graph.node_indices.contains_key("Node2"));
    }

    #[test]
    fn test_clear_connections() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");
        graph.graph.add_edge(*graph.node_indices.get("Node1").unwrap(), *graph.node_indices.get("Node2").unwrap(), ());

        assert_eq!(graph.graph.edge_count(), 1);
        graph.clear_connections();
        assert_eq!(graph.graph.edge_count(), 0);
    }

    #[test]
    fn test_fully_connect_groups() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");
        graph.add_node("Node3");
        graph.fully_connect_groups(2);

        // Since there are 3 nodes and we're grouping them by 2, we expect:
        // - Group 1 (Node1, Node2) to have 1 connection (fully connected pair)
        // - Node3 will be in its own group, with no connections
        assert_eq!(graph.graph.edge_count(), 1);

        // Adding one more node to see it form a fully connected group with Node3
        graph.add_node("Node4");
        graph.fully_connect_groups(2);
        // Now, we expect 2 fully connected pairs: (Node1, Node2) and (Node3, Node4)
        // This means 1 connection per group, total 2 connections.
        assert_eq!(graph.graph.edge_count(), 2);
    }
}