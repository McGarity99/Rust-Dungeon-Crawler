use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11);

const LABYRINTH: (&str, i32, i32) = ("
-------------
-###########-
-#-#-----#M#-
-#---#-#---#-
---#-#M#---#-
-#-#-#-#####-
-#-#-#-#---#-
-#####---#-#-
-#M----#-#-#-
-#########-#-
", 13, 10);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;

    let d_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0
    );

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {    //while placement is empty, and attempts are less than 10, loop
        let dimensions = Rect::with_size(   //create a Rect type starting at a random map location, with the height and width of the vault
            rng.range(0, SCREEN_WIDTH - LABYRINTH.1),
            rng.range(0, SCREEN_HEIGHT - LABYRINTH.2),
            LABYRINTH.1,
            LABYRINTH.2
        );

        let mut can_place = false;
        dimensions.for_each(|pt| {  //iterate every tile in the Rectangle via for_each
            let idx = mb.map.point2d_to_index(pt);
            let distance = d_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {  //if D map distance for the tile is < 2000.0, the tile is reachable
                can_place = true;
            }
        });
        if can_place {
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));    //erase monster locations that fall inside the rectangle with .retain()
        }
        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = LABYRINTH.0
            .chars().filter(|a| *a != '\r' && *a != '\n')   //use an iterator to remove \r and \n charecters in the string literal
            .collect();
        let mut i = 0;  //represents the current location in the prefab as we iterate through it
        for ty in placement.y .. placement.y + LABYRINTH.2 { //iterate every tile the prefab will cover
            for tx in placement.x .. placement.x + LABYRINTH.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];  //retrieve the character at position i from the string
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    },
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    'y' => mb.map.tiles[idx] = TileType::PoisonFloor,
                    _ => println!("No idea what to do with {}", c)
                }
                i += 1;
            }
        }
    }
}