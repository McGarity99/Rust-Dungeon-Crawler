pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,   //helper class from bracket-lib that stores both a foreground and background color in a single struct
    pub glyph: FontCharType //defined in bracket-lib to store a single character/glyph
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32
}  //serves as "tag" indicating that an entity with this component is the player

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Armor {
    pub current: i32,
    pub max: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pub current: i32,
    pub max: i32,
    pub level_theme: i32,
    pub poison_shield: i32,
    pub max_poison_shield: i32
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScoreItem;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FovItem;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TomeOfAnth;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfView {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,    //how many tiles in each direction an entity can see
    pub is_dirty: bool  //dirty optimization pattern
}

impl FieldOfView {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true  //mark new field of view data as dirty - it will be updated the first time the associated system runs
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }

    pub fn inc_fov(&mut self) {
        self.visible_tiles = HashSet::new();
        self.radius += 1;
        self.is_dirty = true;
    }

    pub fn dec_fov(&mut self) {
        self.visible_tiles = HashSet::new();
        self.radius -= FOV_REDUC;
        self.is_dirty = true;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesDungeonMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesNVision {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesArmor {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesScore {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesPoisonR {
    pub amount: i32
}

#[derive(Clone, PartialEq, Debug)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Utility;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IgnoresArmor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AllSeeing;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ReducesFOV;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StealsScore {
    pub amount: i32
}

#[derive(Clone, Debug, PartialEq)]
pub struct LevelTheme(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PoisonShield {
    pub amount: i32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SmallMonster;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LargeMonster;