use crate::prelude::*;
use super::MapArchitect;

pub struct CellularAutomataArchitect{}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }
        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_monsters(&start, rng); //determine locations of monsters
        mb.player_start = start;    //determine player start
        mb.amulet_start = mb.find_most_distant();   //determine amulet position
        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(
        &mut self,
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0,100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..=1 {
                if !(ix == 0 && iy == 0) && map.tiles[map_idx(x+ix, y+iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors as usize
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = map_idx(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH/2, SCREEN_HEIGHT/2);   //ideal center position
        let closest_point = map.tiles
            .iter() //iterate all of the map tiles
            .enumerate()    //give each iteration a tuple of (index, tiletype)
            .filter(|(_, t)| **t == TileType::Floor)    //use filter to remove all tiles that are not floors
            .map(|(idx, _)| (idx, DistanceAlg::Pythagoras.distance2d(   //calculate Pythagorean distance from each tile to the map's center
                center,
                map.index_to_point2d(idx)
            )))
            .min_by(|(_, distance), (_, distance2)| //find lowest distance in the iterator set
                distance.partial_cmp(&distance2).unwrap()
            )
            .map(|(idx, _)| idx)    //use map to transform the iterator from (index, type) into the map indices
            .unwrap();  //min_by returns an Option as there's no guarantee of a lowest value in an iterator set
        map.index_to_point2d(closest_point) //convert the map index to an x,y Point and return it
    }
}