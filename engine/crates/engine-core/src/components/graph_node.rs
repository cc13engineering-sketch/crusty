/// COMPONENT: GraphNode
/// Arbitrary entity-to-entity graph edges with typed labels and weights.

use crate::ecs::Entity;

/// An edge in the entity graph with a typed label and weight.
#[derive(Clone, Debug)]
pub struct GraphEdge {
    pub target: Entity,
    pub edge_type: String,
    pub weight: f64,
    pub bidirectional: bool,
}

/// A node in an arbitrary entity relationship graph.
#[derive(Clone, Debug, Default)]
pub struct GraphNode {
    pub edges: Vec<GraphEdge>,
    pub group: Option<String>,
}

impl GraphNode {
    pub fn new() -> Self { Self::default() }

    pub fn with_group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    pub fn add_edge(&mut self, target: Entity, edge_type: &str, weight: f64, bidirectional: bool) {
        self.edges.push(GraphEdge {
            target,
            edge_type: edge_type.to_string(),
            weight,
            bidirectional,
        });
    }

    pub fn remove_edges_to(&mut self, target: Entity) {
        self.edges.retain(|e| e.target != target);
    }

    pub fn neighbors(&self) -> impl Iterator<Item = Entity> + '_ {
        self.edges.iter().map(|e| e.target)
    }

    pub fn neighbors_of_type<'a>(&'a self, edge_type: &'a str) -> impl Iterator<Item = Entity> + 'a {
        self.edges.iter()
            .filter(move |e| e.edge_type == edge_type)
            .map(|e| e.target)
    }

    pub fn has_edge_to(&self, target: Entity) -> bool {
        self.edges.iter().any(|e| e.target == target)
    }

    pub fn edge_count(&self) -> usize { self.edges.len() }

    pub fn total_weight(&self) -> f64 {
        self.edges.iter().map(|e| e.weight).sum()
    }

    pub fn strongest_edge(&self) -> Option<&GraphEdge> {
        self.edges.iter().max_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_node_empty() {
        let node = GraphNode::new();
        assert_eq!(node.edge_count(), 0);
        assert!(node.group.is_none());
    }

    #[test]
    fn add_and_query_edges() {
        let mut node = GraphNode::new();
        node.add_edge(Entity(1), "supply", 5.0, false);
        node.add_edge(Entity(2), "supply", 3.0, true);
        node.add_edge(Entity(3), "attack", 1.0, false);

        assert_eq!(node.edge_count(), 3);
        assert!(node.has_edge_to(Entity(1)));
        assert!(!node.has_edge_to(Entity(99)));

        let supply_neighbors: Vec<_> = node.neighbors_of_type("supply").collect();
        assert_eq!(supply_neighbors.len(), 2);
    }

    #[test]
    fn remove_edges_to() {
        let mut node = GraphNode::new();
        node.add_edge(Entity(1), "link", 1.0, false);
        node.add_edge(Entity(2), "link", 1.0, false);
        node.remove_edges_to(Entity(1));
        assert_eq!(node.edge_count(), 1);
        assert!(!node.has_edge_to(Entity(1)));
    }

    #[test]
    fn total_weight() {
        let mut node = GraphNode::new();
        node.add_edge(Entity(1), "a", 3.0, false);
        node.add_edge(Entity(2), "b", 7.0, false);
        assert_eq!(node.total_weight(), 10.0);
    }

    #[test]
    fn strongest_edge() {
        let mut node = GraphNode::new();
        node.add_edge(Entity(1), "weak", 1.0, false);
        node.add_edge(Entity(2), "strong", 10.0, false);
        let strongest = node.strongest_edge().unwrap();
        assert_eq!(strongest.target, Entity(2));
    }

    #[test]
    fn group_assignment() {
        let node = GraphNode::new().with_group("colony_a");
        assert_eq!(node.group.as_deref(), Some("colony_a"));
    }

    #[test]
    fn neighbors_iterator() {
        let mut node = GraphNode::new();
        node.add_edge(Entity(5), "x", 1.0, false);
        node.add_edge(Entity(10), "y", 1.0, false);
        let ns: Vec<_> = node.neighbors().collect();
        assert_eq!(ns, vec![Entity(5), Entity(10)]);
    }
}
