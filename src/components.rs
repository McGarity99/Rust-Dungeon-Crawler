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

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AmuletOfYala;

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

    pub fn new_nv() -> Self {
        Self { visible_tiles: HashSet::new(),
             radius: NV_FOV,
              is_dirty: true
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true
        }
    }

    pub fn set_fov(&mut self) {
        self.visible_tiles = HashSet::new();
        self.radius = NV_FOV;
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
pub struct ProvidesNVision;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesArmor {
    pub amount: i32
}

#[derive(Clone, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActivateItem {
    pub used_by: Entity,
    pub item: Entity
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;