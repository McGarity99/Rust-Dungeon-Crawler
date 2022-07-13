mod template;

use template::*;

use crate::prelude::*;

pub fn spawn_player(ecs: &mut World, pos: Point) { //calling this function adds the player and their components to the ECS
    ecs.push(
        (
            Player{map_level: 0},
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            },
            Health {
                current: 15,
                max: 15
            },
            Armor {
                current: 10,
                max: 10
            },
            Score {
                current: 0,
                max: std::i32::MAX,
                level_theme: 0,
                poison_shield: START_P_RESISTANCE,
                max_poison_shield: MAX_P_RESISTANCE
            },
            FieldOfView::new(REG_FOV),
            Damage(1)
        )
    );
}

pub fn spawn_amulet_of_yala(ecs: &mut World, pos: Point) {
    ecs.push(
        (Item, AmuletOfYala,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('|')
            },
            Name("Amulet of Yala".to_string()),
        )
    );
}

pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point]
) {
    let template = Templates::load();
    template.spawn_entities(ecs, rng, level, spawn_points);
}