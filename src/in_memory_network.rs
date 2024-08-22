extern crate petgraph;

use petgraph::algo::kosaraju_scc;
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

#[derive(Default)]
pub struct InMemoryNetwork {
    pub graph: UnGraph<String, ()>,
    // Undirected graph with String as Node data and unit type for edges
    pub node_indices: HashMap<String, NodeIndex>, // Mapping of node data to their NodeIndex for quick access
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

    ///
    ///
    /// # Arguments
    ///
    /// * `node1`:
    /// * `node2`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        self.graph.add_edge(
            *self.node_indices.get(node1).unwrap(),
            *self.node_indices.get(node2).unwrap(),
            (),
        );
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

    /// `get_connected_groups` identifies and returns the connected groups (components)
    /// of nodes in a graph where each group represents a set of nodes that are mutually
    /// reachable from each other. This function is particularly useful for analyzing
    /// networks where you need to understand which nodes can communicate with each other
    /// without requiring connections to other parts of the network.
    ///
    /// # Algorithm:
    /// The function uses the Kosaraju's Strongly Connected Components (SCC) algorithm
    /// to identify connected components in the graph. Although Kosaraju's algorithm is
    /// typically used for finding SCCs in directed graphs, it is versatile enough to
    /// identify connected components in undirected graphs as well.
    ///
    /// 1. **Kosaraju's Algorithm**:
    ///    - The algorithm works in two passes over the graph:
    ///      - **First Pass**: Perform a depth-first search (DFS) on the original graph
    ///        and store nodes in a stack based on their finish times (nodes that finish
    ///        last are pushed first).
    ///      - **Second Pass**: Reverse the directions of all edges in the graph and
    ///        perform DFS using the stack from the first pass. Each DFS in this pass
    ///        identifies a strongly connected component.
    ///    - For undirected graphs, each SCC corresponds to a connected component.
    ///
    /// 2. **Grouping Nodes**:
    ///    - After identifying the connected components (SCCs), the function maps each
    ///      node index to its corresponding component and then groups nodes by their
    ///      component index.
    ///    - The result is a `Vec<Vec<String>>` where each inner vector contains the
    ///      node names of a connected component.
    ///
    /// # Use Case:
    /// This function is useful in scenarios where a graph represents a network of
    /// entities (e.g., computers, people, or services) and you need to determine which
    /// subsets of entities can communicate with each other. For example, in a network
    /// partition simulation, this function could be used to identify isolated groups
    /// of nodes that can only interact within their group.
    ///
    /// # Example:
    /// If you have a graph with nodes 1, 2, 3, 4, 5, and edges such that:
    /// - 1, 2, 3 are connected among themselves
    /// - 4 and 5 are connected among themselves but disconnected from 1, 2, 3
    ///
    /// The function will return `vec![vec!["1", "2", "3"], vec!["4", "5"]]`,
    /// representing the two connected groups in the network.
    ///
    /// # Returns:
    /// - A vector of vectors, where each inner vector represents a group of node names
    ///   that are connected within the graph.
    pub fn get_connected_groups(&self) -> Vec<Vec<String>> {
        // Get the connected components using the kosaraju_scc algorithm for directed graphs
        // This returns a Vec<Vec<NodeIndex>> where each Vec<NodeIndex> represents a connected component
        let scc = kosaraju_scc(&self.graph);

        // Convert the NodeIndex groups to the actual node names
        scc.iter()
            .map(|component| {
                component
                    .iter()
                    .map(|&node_index| self.graph.node_weight(node_index).unwrap().clone())
                    .collect()
            })
            .collect()
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
        graph.add_edge("Node1", "Node2");

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

    #[test]
    fn test_nodes_are_connected() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");
        graph.add_node("Node3");
        graph.fully_connect_groups(3);

        let connected = graph.are_connected("Node1", "Node2");
        assert!(connected);
        let connected = graph.are_connected("Node1", "Node3");
        assert!(connected);
        let connected = graph.are_connected("Node2", "Node3");
        assert!(connected);
    }

    #[test]
    fn test_no_connections() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");

        assert!(!graph.are_connected("Node1", "Node2"));

        let groups = graph.get_connected_groups();
        assert_eq!(groups.len(), 2);
        // Since the order of groups is not guaranteed, we check for membership rather than direct comparison
        assert!(
            groups.contains(&vec!["Node1".to_string()])
                || groups.contains(&vec!["Node2".to_string()])
        );
    }

    #[test]
    fn test_fully_connected_graph() {
        let mut graph = InMemoryNetwork::new();
        graph.add_node("Node1");
        graph.add_node("Node2");
        graph.fully_connect_groups(2); // Fully connect all nodes

        let groups = graph.get_connected_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
        assert!(
            groups[0].contains(&"Node1".to_string()) && groups[0].contains(&"Node2".to_string())
        );
    }
}
