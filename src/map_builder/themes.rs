use crate::prelude::*;

pub struct DungeonTheme{}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::PoisonFloor => to_cp437('-'),
            TileType::Decorative => to_cp437('M'),
            TileType::OtherDecorative => to_cp437('3'),
            TileType::Door => to_cp437('4'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct ForestTheme{}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::PoisonFloor => to_cp437('y'),
            TileType::Decorative => to_cp437('$'),
            TileType::OtherDecorative => to_cp437('%'),
            TileType::Door => to_cp437('2'),
            TileType::Exit => to_cp437('>')
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

pub struct VolcanoTheme{}

impl MapTheme for VolcanoTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('G'),
            TileType::Wall => to_cp437('I'),
            TileType::PoisonFloor => to_cp437('J'),
            TileType::Decorative => to_cp437('H'),
            TileType::OtherDecorative => to_cp437('X'),
            TileType::Door => to_cp437('Y'),
            TileType::Exit => to_cp437('>')
        }
    }
}

impl VolcanoTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}

pub struct TempleTheme{}

impl MapTheme for TempleTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        let mut rng = RandomNumberGenerator::new();
        match tile_type {
            TileType::Floor => to_cp437(')'),
            TileType::Wall => to_cp437('('),
            TileType::PoisonFloor => to_cp437('*'),
            TileType::Decorative => to_cp437('='),
            TileType::OtherDecorative => to_cp437('+'),
            TileType::Door => to_cp437('8'),
            TileType::Exit => to_cp437('>')
        }
    }
}

impl TempleTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}