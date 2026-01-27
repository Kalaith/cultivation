use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// An economy node (market/trade hub) in the world
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EconomyNode {
    pub id: String,
    pub name: String,
    /// Links to MapNode
    pub map_node_id: String,
    /// Faction that controls this market
    #[serde(default)]
    pub controller_faction_id: String,
    /// Item ID -> base price in spirit stones
    #[serde(default)]
    pub base_prices: HashMap<String, u32>,
    /// Current stock levels by item ID
    #[serde(default)]
    pub supply: HashMap<String, u32>,
    /// Demand multipliers by item ID (1.0 = normal)
    #[serde(default)]
    pub demand: HashMap<String, f32>,
}

impl EconomyNode {
    /// Calculate current price for an item
    pub fn get_price(&self, item_id: &str, elasticity: f32) -> Option<u32> {
        let base_price = self.base_prices.get(item_id)?;
        let demand_mod = self.demand.get(item_id).copied().unwrap_or(1.0);
        let supply_amount = self.supply.get(item_id).copied().unwrap_or(100) as f32;
        let max_supply = 100.0; // Normalized supply ratio
        let supply_ratio = (supply_amount / max_supply).clamp(0.1, 2.0);

        // Price = base * demand * (1 + (1 - supply_ratio) * elasticity)
        let price = *base_price as f32 * demand_mod * (1.0 + (1.0 - supply_ratio) * elasticity);
        Some(price.max(1.0) as u32)
    }

    /// Reduce supply when item is purchased
    pub fn reduce_supply(&mut self, item_id: &str, amount: u32) {
        if let Some(supply) = self.supply.get_mut(item_id) {
            *supply = supply.saturating_sub(amount);
        }
    }

    /// Increase supply when item is sold or regenerates
    pub fn increase_supply(&mut self, item_id: &str, amount: u32) {
        *self.supply.entry(item_id.to_string()).or_insert(0) += amount;
    }

    /// Regenerate supply over time
    pub fn regenerate_supply(&mut self, regen_rate: f32) {
        for (_, supply) in self.supply.iter_mut() {
            let regen = (*supply as f32 * regen_rate).max(1.0) as u32;
            *supply = (*supply + regen).min(200); // Cap at 200% normal supply
        }
    }
}

/// A trade route connecting two economy nodes
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeRoute {
    pub id: String,
    pub from_node: String,
    pub to_node: String,
    /// 0.0 (very dangerous) to 1.0 (perfectly safe)
    #[serde(default = "default_safety")]
    pub safety_rating: f32,
    /// Ticks to travel the route
    pub travel_ticks: u32,
    /// Whether the route is currently operational
    #[serde(default = "default_true")]
    pub active: bool,
}

fn default_safety() -> f32 {
    1.0
}

fn default_true() -> bool {
    true
}

impl TradeRoute {
    /// Calculate trade volume modifier based on safety
    pub fn trade_volume_modifier(&self) -> f32 {
        if !self.active {
            return 0.0;
        }
        self.safety_rating.clamp(0.0, 1.0)
    }

    /// Check if route is disrupted
    pub fn is_disrupted(&self) -> bool {
        !self.active || self.safety_rating < 0.3
    }
}

/// Temporary price modifier for an item
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceModifier {
    pub item_id: String,
    /// 0.5 = 50% cheaper, 1.5 = 50% more expensive
    pub modifier: f32,
    pub reason: String,
    /// Tick when this modifier expires (None = permanent)
    pub expires_tick: Option<u64>,
}

impl PriceModifier {
    pub fn new(item_id: String, modifier: f32, reason: String, duration: Option<u32>, current_tick: u64) -> Self {
        Self {
            item_id,
            modifier,
            reason,
            expires_tick: duration.map(|d| current_tick + d as u64),
        }
    }

    pub fn is_expired(&self, current_tick: u64) -> bool {
        self.expires_tick.map(|t| current_tick >= t).unwrap_or(false)
    }
}

/// Trade transaction record
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeTransaction {
    pub economy_node_id: String,
    pub item_id: String,
    pub amount: u32,
    pub price_per_unit: u32,
    pub is_purchase: bool, // true = player bought, false = player sold
    pub tick: u64,
}

/// Player's active trade caravan
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeCaravan {
    pub id: u64,
    pub route_id: String,
    pub goods: HashMap<String, u32>,
    pub ticks_remaining: u32,
    pub departed_tick: u64,
}

impl TradeCaravan {
    /// Check if caravan has arrived
    pub fn has_arrived(&self) -> bool {
        self.ticks_remaining == 0
    }

    /// Progress the caravan by one tick
    pub fn tick(&mut self) {
        if self.ticks_remaining > 0 {
            self.ticks_remaining -= 1;
        }
    }
}

/// Economic state tracking for the world
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EconomyState {
    pub nodes: Vec<EconomyNode>,
    pub routes: Vec<TradeRoute>,
    pub price_modifiers: Vec<PriceModifier>,
    pub active_caravans: Vec<TradeCaravan>,
    pub transaction_history: Vec<TradeTransaction>,
    pub next_caravan_id: u64,
}

impl EconomyState {
    pub fn new(nodes: Vec<EconomyNode>, routes: Vec<TradeRoute>) -> Self {
        Self {
            nodes,
            routes,
            price_modifiers: Vec::new(),
            active_caravans: Vec::new(),
            transaction_history: Vec::new(),
            next_caravan_id: 1,
        }
    }

    /// Get an economy node by ID
    pub fn get_node(&self, id: &str) -> Option<&EconomyNode> {
        self.nodes.iter().find(|n| n.id == id)
    }

    /// Get a mutable economy node by ID
    pub fn get_node_mut(&mut self, id: &str) -> Option<&mut EconomyNode> {
        self.nodes.iter_mut().find(|n| n.id == id)
    }

    /// Get a trade route by ID
    pub fn get_route(&self, id: &str) -> Option<&TradeRoute> {
        self.routes.iter().find(|r| r.id == id)
    }

    /// Get all routes from a node
    pub fn get_routes_from(&self, node_id: &str) -> Vec<&TradeRoute> {
        self.routes.iter().filter(|r| r.from_node == node_id && r.active).collect()
    }

    /// Add a price modifier
    pub fn add_price_modifier(&mut self, modifier: PriceModifier) {
        self.price_modifiers.push(modifier);
    }

    /// Clean up expired price modifiers
    pub fn cleanup_expired_modifiers(&mut self, current_tick: u64) {
        self.price_modifiers.retain(|m| !m.is_expired(current_tick));
    }

    /// Get effective price for an item at a node
    pub fn get_effective_price(&self, node_id: &str, item_id: &str, elasticity: f32) -> Option<u32> {
        let node = self.get_node(node_id)?;
        let base_price = node.get_price(item_id, elasticity)?;

        // Apply all active price modifiers
        let modifier: f32 = self.price_modifiers
            .iter()
            .filter(|m| m.item_id == item_id)
            .map(|m| m.modifier)
            .product();

        Some((base_price as f32 * modifier).max(1.0) as u32)
    }

    /// Process economy tick
    pub fn tick(&mut self, current_tick: u64, regen_rate: f32) {
        // Regenerate supply at all nodes
        for node in &mut self.nodes {
            node.regenerate_supply(regen_rate);
        }

        // Progress caravans
        for caravan in &mut self.active_caravans {
            caravan.tick();
        }

        // Cleanup expired modifiers
        self.cleanup_expired_modifiers(current_tick);
    }

    /// Get arrived caravans and remove them
    pub fn collect_arrived_caravans(&mut self) -> Vec<TradeCaravan> {
        let (arrived, still_traveling): (Vec<_>, Vec<_>) = self.active_caravans
            .drain(..)
            .partition(|c| c.has_arrived());
        self.active_caravans = still_traveling;
        arrived
    }
}
