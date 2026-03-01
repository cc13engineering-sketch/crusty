/// COMPONENT: ResourceInventory
/// Bounded multi-resource container with production, consumption, and transfer.

use std::collections::HashMap;

/// A single resource slot with capacity and passive rates.
#[derive(Clone, Debug)]
pub struct ResourceSlot {
    pub current: f64,
    pub capacity: f64,
    pub production_rate: f64,
    pub consumption_rate: f64,
}

impl ResourceSlot {
    pub fn new(capacity: f64) -> Self {
        Self { current: 0.0, capacity, production_rate: 0.0, consumption_rate: 0.0 }
    }

    pub fn with_amount(mut self, amount: f64) -> Self {
        self.current = amount.min(self.capacity);
        self
    }

    pub fn with_production(mut self, rate: f64) -> Self {
        self.production_rate = rate;
        self
    }

    pub fn with_consumption(mut self, rate: f64) -> Self {
        self.consumption_rate = rate;
        self
    }

    /// Deposit up to `amount`, returns actual deposited.
    pub fn deposit(&mut self, amount: f64) -> f64 {
        let space = self.capacity - self.current;
        let actual = amount.min(space).max(0.0);
        self.current += actual;
        actual
    }

    /// Withdraw up to `amount`, returns actual withdrawn.
    pub fn withdraw(&mut self, amount: f64) -> f64 {
        let actual = amount.min(self.current).max(0.0);
        self.current -= actual;
        actual
    }

    pub fn fill_ratio(&self) -> f64 {
        if self.capacity <= 0.0 { 0.0 } else { self.current / self.capacity }
    }

    pub fn is_full(&self) -> bool { self.current >= self.capacity - f64::EPSILON }
    pub fn is_empty(&self) -> bool { self.current <= f64::EPSILON }
}

/// Per-entity multi-resource inventory.
#[derive(Clone, Debug, Default)]
pub struct ResourceInventory {
    pub slots: HashMap<String, ResourceSlot>,
}

impl ResourceInventory {
    pub fn new() -> Self { Self::default() }

    pub fn with_slot(mut self, name: &str, slot: ResourceSlot) -> Self {
        self.slots.insert(name.to_string(), slot);
        self
    }

    pub fn add_slot(&mut self, name: &str, slot: ResourceSlot) {
        self.slots.insert(name.to_string(), slot);
    }

    pub fn get(&self, name: &str) -> Option<&ResourceSlot> { self.slots.get(name) }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut ResourceSlot> { self.slots.get_mut(name) }

    pub fn amount(&self, name: &str) -> f64 {
        self.slots.get(name).map_or(0.0, |s| s.current)
    }

    pub fn has_at_least(&self, name: &str, amount: f64) -> bool {
        self.amount(name) >= amount
    }

    /// Tick production and consumption for dt seconds.
    pub fn tick(&mut self, dt: f64) {
        for slot in self.slots.values_mut() {
            if slot.production_rate > 0.0 {
                let produced = slot.production_rate * dt;
                slot.current = (slot.current + produced).min(slot.capacity);
            }
            if slot.consumption_rate > 0.0 {
                let consumed = slot.consumption_rate * dt;
                slot.current = (slot.current - consumed).max(0.0);
            }
        }
    }

    pub fn slot_count(&self) -> usize { self.slots.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_deposit_within_capacity() {
        let mut slot = ResourceSlot::new(100.0).with_amount(30.0);
        let deposited = slot.deposit(50.0);
        assert_eq!(deposited, 50.0);
        assert_eq!(slot.current, 80.0);
    }

    #[test]
    fn slot_deposit_overflow_clamped() {
        let mut slot = ResourceSlot::new(100.0).with_amount(80.0);
        let deposited = slot.deposit(50.0);
        assert_eq!(deposited, 20.0);
        assert_eq!(slot.current, 100.0);
    }

    #[test]
    fn slot_withdraw_available() {
        let mut slot = ResourceSlot::new(100.0).with_amount(50.0);
        let withdrawn = slot.withdraw(30.0);
        assert_eq!(withdrawn, 30.0);
        assert_eq!(slot.current, 20.0);
    }

    #[test]
    fn slot_withdraw_insufficient() {
        let mut slot = ResourceSlot::new(100.0).with_amount(10.0);
        let withdrawn = slot.withdraw(30.0);
        assert_eq!(withdrawn, 10.0);
        assert!((slot.current).abs() < f64::EPSILON);
    }

    #[test]
    fn slot_fill_ratio() {
        let slot = ResourceSlot::new(200.0).with_amount(100.0);
        assert_eq!(slot.fill_ratio(), 0.5);
    }

    #[test]
    fn inventory_multi_resource() {
        let inv = ResourceInventory::new()
            .with_slot("water", ResourceSlot::new(100.0).with_amount(50.0))
            .with_slot("nutrients", ResourceSlot::new(50.0).with_amount(10.0));
        assert_eq!(inv.amount("water"), 50.0);
        assert_eq!(inv.amount("nutrients"), 10.0);
        assert_eq!(inv.amount("missing"), 0.0);
        assert_eq!(inv.slot_count(), 2);
    }

    #[test]
    fn inventory_has_at_least() {
        let inv = ResourceInventory::new()
            .with_slot("gold", ResourceSlot::new(100.0).with_amount(50.0));
        assert!(inv.has_at_least("gold", 30.0));
        assert!(!inv.has_at_least("gold", 60.0));
    }

    #[test]
    fn inventory_tick_production() {
        let mut inv = ResourceInventory::new()
            .with_slot("food", ResourceSlot::new(100.0).with_amount(0.0).with_production(10.0));
        inv.tick(1.0);
        assert_eq!(inv.amount("food"), 10.0);
    }

    #[test]
    fn inventory_tick_consumption() {
        let mut inv = ResourceInventory::new()
            .with_slot("energy", ResourceSlot::new(100.0).with_amount(50.0).with_consumption(20.0));
        inv.tick(1.0);
        assert_eq!(inv.amount("energy"), 30.0);
    }

    #[test]
    fn inventory_tick_consumption_floor() {
        let mut inv = ResourceInventory::new()
            .with_slot("mana", ResourceSlot::new(100.0).with_amount(5.0).with_consumption(100.0));
        inv.tick(1.0);
        assert_eq!(inv.amount("mana"), 0.0);
    }

    #[test]
    fn inventory_tick_production_cap() {
        let mut inv = ResourceInventory::new()
            .with_slot("hp", ResourceSlot::new(100.0).with_amount(95.0).with_production(50.0));
        inv.tick(1.0);
        assert_eq!(inv.amount("hp"), 100.0);
    }
}
