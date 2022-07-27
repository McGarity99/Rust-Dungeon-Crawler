use crate::prelude::*;
use super::MapArchitect;
pub struct DrunkardWalkArchitect{}

const STAGGER_DIST: usize = 400;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;
const DESIRED_FLOOR: usize = NUM_TILES / 3;

impl MapArchitect for DrunkardWalkArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            tome_start: Point::zero(),
            theme: super::themes::DungeonTheme::new()
        };
        mb.fill(TileType::Wall);
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);
        while mb.map.tiles.iter()
            .filter(|t| **t == TileType::Floor).count() < DESIRED_FLOOR //while the map is incomplete, keep adding miners
        {
            self.drunkard(
                &Point::new(
                    rng.range(0, SCREEN_WIDTH),
                    rng.range(0, SCREEN_HEIGHT)
                ),
                rng,
                &mut mb.map
            );  //start miner on random location in map
            let dijkstra_map = DijkstraMap::new(    //build D map for eliminating inaccessible areas
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0
            );
            dijkstra_map.map
                .iter() //iterate the D map results
                .enumerate()    //use enumerate() to add a tile index to each entry in the iterator
                .filter(|(_, distance)| *distance > &2000.0)    //use filter to retain values with a distance over 2000 tiles from the start point (inaccessible)
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);   //for each remaining entry in the iterator, convert it to a wall
        }
        mb.monster_spawns = mb.spawn_monsters(&center, rng);
        mb.player_start = center;
        mb.tome_start = mb.find_most_distant();
        mb
    }
}

impl DrunkardWalkArchitect {
    fn drunkard(
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        let mut drunkard_pos = start.clone();
        let mut distance_staggered = 0;
        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;

            match rng.range(0,4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1
            }
            if !map.in_bounds(drunkard_pos) {
                break;
            }
            distance_staggered += 1;
            if distance_staggered >= STAGGER_DIST {
                break;
            }
        }
    }
}