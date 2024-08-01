use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub ambient_light: f32,
    bed_works: i32,
    coordinate_scale: f64,
    effects: DimensionEffects,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_time: Option<i32>,
    has_ceiling: i32,
    has_raids: i32,
    has_skylight: i32,
    height: i32,
    infiniburn: String,
    logical_height: i32,
    min_y: i32,
    monster_spawn_block_light_limit: i32,
    monster_spawn_light_level: MonsterSpawnLightLevel,
    natural: i32,
    piglin_safe: i32,
    respawn_anchor_works: i32,
    ultrawarm: i32,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum DimensionEffects {
    #[serde(rename = "minecraft:overworld")]
    #[default]
    Overworld,
    #[serde(rename = "minecraft:the_nether")]
    TheNether,
    #[serde(rename = "minecraft:the_end")]
    TheEnd,
}

impl Default for Dimension {
    fn default() -> Self {
        Self {
            ambient_light: 0.0,
            bed_works: 1,
            coordinate_scale: 1.0,
            effects: DimensionEffects::default(),
            fixed_time: None,
            has_ceiling: 0,
            has_raids: 1,
            has_skylight: 1,
            height: 384,
            infiniburn: "#minecraft:infiniburn_overworld".into(),
            logical_height: 384,
            min_y: -64,
            monster_spawn_block_light_limit: 0,
            monster_spawn_light_level: MonsterSpawnLightLevel::Int(7),
            natural: 1,
            piglin_safe: 0,
            respawn_anchor_works: 0,
            ultrawarm: 0,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonsterSpawnLightLevel {
    Int(i32),
    Tagged(MonsterSpawnLightLevelTagged),
}

#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum MonsterSpawnLightLevelTagged {
    #[serde(rename = "minecraft:uniform")]
    Uniform {
        min_inclusive: i32,
        max_inclusive: i32,
    },
}

impl From<i32> for MonsterSpawnLightLevel {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}