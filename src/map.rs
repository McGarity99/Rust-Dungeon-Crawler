use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    PoisonFloor,
    Decorative,
    OtherDecorative,
    Door,
    Exit
}


pub struct Map {        //map indexed Y-first
    pub tiles: Vec<TileType>,
    pub revealed_tiles: Vec<bool>,
    pub key_carried: bool,
    pub spawned_points: Vec<usize>, //vec of points where entities are spawned
    pub prefab_indices: Vec<usize>  //vec of points that are within a prefab structure
}

impl Map {
    pub fn new() -> Self {
        Self { 
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed_tiles: vec![false; NUM_TILES],
            key_carried: false,
            spawned_points: Vec::new(),
            prefab_indices: Vec::new() 
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH
            && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tiles[map_idx(point.x, point.y)] == TileType::Floor ||
            self.tiles[map_idx(point.x, point.y)] == TileType::PoisonFloor ||
            self.tiles[map_idx(point.x, point.y)] == TileType::Exit ||
            (self.tiles[map_idx(point.x, point.y)] == TileType::Door && self.key_carried))
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
    
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1,0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras
            .distance2d(
                self.index_to_point2d(idx1),
                self.index_to_point2d(idx2)
            )
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor &&  //FOV extends through Floor tiles
        self.tiles[idx as usize] != TileType::PoisonFloor   //FOV extends through PoisonFloor tiles
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    return ((y * SCREEN_WIDTH) + x) as usize;   //return the index represented by x,y coordinates
}