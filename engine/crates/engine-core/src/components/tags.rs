use super::SchemaInfo;

/// Compiler-enforced entity tags. Using an enum instead of strings catches
/// typos at compile time and enables exhaustive matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tag {
    Player,
    Bullet,
    Enemy,
    Asteroid,
    Ball,
    Spawner,
    Node,
    Root,
    Goal,
    EncounterZone,
    Pickup,
    Transition,
    Trainer,
    Boss,
    HealthPickup,
    Test,
    Doomed,
    Npc,
}

impl From<&str> for Tag {
    fn from(s: &str) -> Self {
        Tag::from_str(s).unwrap_or(Tag::Test)
    }
}

impl Tag {
    /// Parse from a string (for .world file loading / JSON interop).
    pub fn from_str(s: &str) -> Option<Tag> {
        match s {
            "player" => Some(Tag::Player),
            "bullet" => Some(Tag::Bullet),
            "enemy" => Some(Tag::Enemy),
            "asteroid" => Some(Tag::Asteroid),
            "ball" => Some(Tag::Ball),
            "spawner" => Some(Tag::Spawner),
            "node" => Some(Tag::Node),
            "root" => Some(Tag::Root),
            "goal" => Some(Tag::Goal),
            "encounter_zone" => Some(Tag::EncounterZone),
            "pickup" => Some(Tag::Pickup),
            "transition" => Some(Tag::Transition),
            "trainer" => Some(Tag::Trainer),
            "boss" => Some(Tag::Boss),
            "health_pickup" => Some(Tag::HealthPickup),
            "test" => Some(Tag::Test),
            "doomed" => Some(Tag::Doomed),
            "npc" => Some(Tag::Npc),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Tag::Player => "player",
            Tag::Bullet => "bullet",
            Tag::Enemy => "enemy",
            Tag::Asteroid => "asteroid",
            Tag::Ball => "ball",
            Tag::Spawner => "spawner",
            Tag::Node => "node",
            Tag::Root => "root",
            Tag::Goal => "goal",
            Tag::EncounterZone => "encounter_zone",
            Tag::Pickup => "pickup",
            Tag::Transition => "transition",
            Tag::Trainer => "trainer",
            Tag::Boss => "boss",
            Tag::HealthPickup => "health_pickup",
            Tag::Test => "test",
            Tag::Doomed => "doomed",
            Tag::Npc => "npc",
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Tags {
    pub values: Vec<Tag>,
}

impl Tags {
    pub fn has(&self, tag: Tag) -> bool {
        self.values.contains(&tag)
    }
    /// Check by string name (for data-driven rules, camera targets, etc.)
    pub fn has_str(&self, tag: &str) -> bool {
        Tag::from_str(tag).map_or(false, |t| self.values.contains(&t))
    }
    pub fn new(tags: &[&str]) -> Self {
        Self {
            values: tags.iter().filter_map(|s| Tag::from_str(s)).collect(),
        }
    }
    pub fn from_tags(tags: &[Tag]) -> Self {
        Self { values: tags.to_vec() }
    }
}

impl SchemaInfo for Tags {
    fn schema_name() -> &'static str { "Tags" }
    fn schema() -> serde_json::Value {
        serde_json::json!({ "fields": { "values": { "type": "Vec<Tag>" } } })
    }
}
