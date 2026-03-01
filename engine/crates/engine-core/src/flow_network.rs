/// ENGINE MODULE: FlowNetwork
/// Directed graph of resource flow edges between entities.
/// Each frame, resources are transferred along edges based on capacity,
/// priority, and available supply/demand.

use crate::ecs::Entity;

/// A directed edge in the flow network.
#[derive(Clone, Debug)]
pub struct FlowEdge {
    pub source: Entity,
    pub target: Entity,
    pub resource: String,
    pub max_rate: f64,
    pub current_rate: f64,
    pub priority: i32,
    pub enabled: bool,
}

/// The flow network: edges processed each frame to transfer resources.
#[derive(Default, Debug)]
pub struct FlowNetwork {
    pub edges: Vec<FlowEdge>,
}

impl FlowNetwork {
    pub fn new() -> Self { Self::default() }

    pub fn add_edge(
        &mut self, source: Entity, target: Entity,
        resource: &str, max_rate: f64, priority: i32,
    ) {
        self.edges.push(FlowEdge {
            source, target,
            resource: resource.to_string(),
            max_rate, current_rate: 0.0,
            priority, enabled: true,
        });
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.edges.retain(|e| e.source != entity && e.target != entity);
    }

    pub fn edges_from(&self, entity: Entity) -> impl Iterator<Item = &FlowEdge> + '_ {
        self.edges.iter().filter(move |e| e.source == entity)
    }

    pub fn edges_to(&self, entity: Entity) -> impl Iterator<Item = &FlowEdge> + '_ {
        self.edges.iter().filter(move |e| e.target == entity)
    }

    pub fn edge_count(&self) -> usize { self.edges.len() }

    pub fn clear(&mut self) { self.edges.clear(); }

    /// Solve resource flow for one timestep.
    /// Transfers resources between entity inventories along edges.
    pub fn solve(&mut self, world: &mut crate::ecs::World, dt: f64) {
        if dt <= 0.0 { return; }

        // Sort by priority (higher priority first)
        let mut indices: Vec<usize> = (0..self.edges.len()).collect();
        indices.sort_by(|&a, &b| self.edges[b].priority.cmp(&self.edges[a].priority));

        for &idx in &indices {
            let edge = &self.edges[idx];
            if !edge.enabled { continue; }

            let max_transfer = edge.max_rate * dt;
            let source = edge.source;
            let target = edge.target;

            // Check source availability
            let available = world.resource_inventories.get(source)
                .and_then(|inv| inv.get(&edge.resource))
                .map_or(0.0, |slot| slot.current);

            // Check target capacity
            let space = world.resource_inventories.get(target)
                .and_then(|inv| inv.get(&edge.resource))
                .map_or(0.0, |slot| slot.capacity - slot.current);

            let actual = max_transfer.min(available).min(space).max(0.0);

            if actual > 0.0 {
                // Clone the resource key once, only when we actually need to mutate
                let resource = self.edges[idx].resource.clone();
                if let Some(inv) = world.resource_inventories.get_mut(source) {
                    if let Some(slot) = inv.get_mut(&resource) {
                        slot.withdraw(actual);
                    }
                }
                if let Some(inv) = world.resource_inventories.get_mut(target) {
                    if let Some(slot) = inv.get_mut(&resource) {
                        slot.deposit(actual);
                    }
                }
            }

            self.edges[idx].current_rate = actual / dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ecs::World;
    use crate::components::resource_inventory::{ResourceInventory, ResourceSlot};

    #[test]
    fn add_and_remove_edges() {
        let mut net = FlowNetwork::new();
        net.add_edge(Entity(1), Entity(2), "water", 10.0, 0);
        net.add_edge(Entity(2), Entity(3), "water", 5.0, 0);
        assert_eq!(net.edge_count(), 2);
        net.remove_entity(Entity(2));
        assert_eq!(net.edge_count(), 0);
    }

    #[test]
    fn edges_from_and_to() {
        let mut net = FlowNetwork::new();
        net.add_edge(Entity(1), Entity(2), "a", 1.0, 0);
        net.add_edge(Entity(1), Entity(3), "b", 1.0, 0);
        net.add_edge(Entity(2), Entity(3), "c", 1.0, 0);

        let from_1: Vec<_> = net.edges_from(Entity(1)).collect();
        assert_eq!(from_1.len(), 2);
        let to_3: Vec<_> = net.edges_to(Entity(3)).collect();
        assert_eq!(to_3.len(), 2);
    }

    #[test]
    fn solve_transfers_resources() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();

        world.resource_inventories.insert(a,
            ResourceInventory::new()
                .with_slot("water", ResourceSlot::new(100.0).with_amount(50.0)));
        world.resource_inventories.insert(b,
            ResourceInventory::new()
                .with_slot("water", ResourceSlot::new(100.0).with_amount(0.0)));

        let mut net = FlowNetwork::new();
        net.add_edge(a, b, "water", 10.0, 0);

        net.solve(&mut world, 1.0);

        assert_eq!(world.resource_inventories.get(a).unwrap().amount("water"), 40.0);
        assert_eq!(world.resource_inventories.get(b).unwrap().amount("water"), 10.0);
    }

    #[test]
    fn solve_respects_capacity() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();

        world.resource_inventories.insert(a,
            ResourceInventory::new()
                .with_slot("food", ResourceSlot::new(100.0).with_amount(100.0)));
        world.resource_inventories.insert(b,
            ResourceInventory::new()
                .with_slot("food", ResourceSlot::new(5.0).with_amount(3.0)));

        let mut net = FlowNetwork::new();
        net.add_edge(a, b, "food", 100.0, 0);

        net.solve(&mut world, 1.0);

        assert_eq!(world.resource_inventories.get(b).unwrap().amount("food"), 5.0);
        assert_eq!(world.resource_inventories.get(a).unwrap().amount("food"), 98.0);
    }

    #[test]
    fn solve_priority_order() {
        let mut world = World::new();
        let src = world.spawn();
        let high_pri = world.spawn();
        let low_pri = world.spawn();

        world.resource_inventories.insert(src,
            ResourceInventory::new()
                .with_slot("gold", ResourceSlot::new(100.0).with_amount(10.0)));
        world.resource_inventories.insert(high_pri,
            ResourceInventory::new()
                .with_slot("gold", ResourceSlot::new(100.0)));
        world.resource_inventories.insert(low_pri,
            ResourceInventory::new()
                .with_slot("gold", ResourceSlot::new(100.0)));

        let mut net = FlowNetwork::new();
        net.add_edge(src, low_pri, "gold", 100.0, 0);
        net.add_edge(src, high_pri, "gold", 100.0, 10);

        net.solve(&mut world, 1.0);

        // High priority should get all 10 units
        assert_eq!(world.resource_inventories.get(high_pri).unwrap().amount("gold"), 10.0);
        assert_eq!(world.resource_inventories.get(low_pri).unwrap().amount("gold"), 0.0);
    }

    #[test]
    fn disabled_edge_no_transfer() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();

        world.resource_inventories.insert(a,
            ResourceInventory::new()
                .with_slot("x", ResourceSlot::new(100.0).with_amount(50.0)));
        world.resource_inventories.insert(b,
            ResourceInventory::new()
                .with_slot("x", ResourceSlot::new(100.0)));

        let mut net = FlowNetwork::new();
        net.add_edge(a, b, "x", 10.0, 0);
        net.edges[0].enabled = false;

        net.solve(&mut world, 1.0);

        assert_eq!(world.resource_inventories.get(a).unwrap().amount("x"), 50.0);
        assert_eq!(world.resource_inventories.get(b).unwrap().amount("x"), 0.0);
    }

    #[test]
    fn solve_missing_inventory_safe() {
        let mut world = World::new();
        let a = world.spawn();
        let b = world.spawn();
        // No inventories added

        let mut net = FlowNetwork::new();
        net.add_edge(a, b, "x", 10.0, 0);
        net.solve(&mut world, 1.0); // Should not panic
    }
}
