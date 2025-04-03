/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {  
        let (from_node, to_node, weight) = edge;  
        
        // 确保两个节点都存在  
        self.add_node(from_node);  
        self.add_node(to_node);  
        
        // 添加从from_node到to_node的边  
        if let Some(neighbors) = self.adjacency_table_mutable().get_mut(from_node) {  
            // 检查边是否已存在，如果存在则更新权重  
            if let Some(existing_edge) = neighbors.iter_mut().find(|(node, _)| node == to_node) {  
                existing_edge.1 = weight;  
            } else {  
                // 如果边不存在，添加新边  
                neighbors.push((to_node.to_string(), weight));  
            }  
        }  
        
        // 添加从to_node到from_node的边（因为是无向图）  
        if let Some(neighbors) = self.adjacency_table_mutable().get_mut(to_node) {  
            // 检查边是否已存在，如果存在则更新权重  
            if let Some(existing_edge) = neighbors.iter_mut().find(|(node, _)| node == from_node) {  
                existing_edge.1 = weight;  
            } else {  
                // 如果边不存在，添加新边  
                neighbors.push((from_node.to_string(), weight));  
            }  
        }  
    }  
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {  
        if self.contains(node) {  
            // 节点已存在，返回false表示没有添加新节点  
            return false;  
        }  
        
        // 添加节点到邻接表中，初始化为空的邻居列表  
        self.adjacency_table_mutable().insert(node.to_string(), Vec::new());  
        true  
    }  
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}