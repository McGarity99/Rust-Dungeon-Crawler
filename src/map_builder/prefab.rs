use crate::prelude::*;

const FORTRESS: (&str, i32, i32) = ("
------------
---######---
---d----#---
---#-M--#---
-###--y-###-
-D--M-y---D-
-###--yy###-
---#----#---
---#--M-#---
---###d##---
------------
", 12, 11);

const FORTRESS_1: (&str, i32, i32) = ("
------------
---###d##---
---#----#---
---#-M--#---
-#d#y--y###-
-D---##---D-
-###y--y#d#-
---#----#---
---d-M--d---
---######---
------------
", 12, 11);

const LABYRINTH: (&str, i32, i32) = ("
-------------
-###########-
-#-#--y--#M#-
-#---d-#-y-#-
-D-#-#M#-y-#-
-#-#y#-##d##-
-#-#-#-#---#-
-#d###---#-#-
-#M-y--#-#-#-
-#########D#-
-------------
", 13, 11);

const LABYRINTH_1: (&str, i32, i32) = ("
-------------
-#####d#####-
-#-#-----#M#-
-#-M-#y#---#-
-D-#-#y#---#-
-#-#-#y###d#-
-#-#-#y#---#-
-##d##---#-#-
-#---M-#-#-#-
-d########D#-
-------------
", 13, 11);

const MAZE: (&str, i32, i32) = ("
-----------------
-#######D#######-
-##y------y--###-
-###-d#-##-#-###-
-###-#####-#####-
-##-y---M--##d##-
-##-########y-##-
-##-##---#-----#-
-#M-y--#---###M#-
-###dd##D#######-
-----------------
", 17, 11);

const MAZE_1: (&str, i32, i32) = ("
-----------------
-##d###d####D###-
-##-M-----y--###-
-###-d#-##-#-###-
-d##y#####-#####-
-##-yy--M--##d##-
-d#-########yy##-
-##-##---#--M--#-
-#M-y--#---d##M#-
-###d#####D#####-
-----------------
", 17, 11);

const TOMB: (&str, i32, i32) = ("
-----------
-d#D#D#D##-
-#--#-#-M#-
-#-#---#-#-
-#y-yyy-yd-
-#---M---#-
-#---M---#-
-##d#-####-
-#y#-y-#y#-
-#M-#-#--#-
-##D#D#D##-
-----------
", 11, 12);

const TOMB_1: (&str, i32, i32) = ("
-----------
-##D#D#D##-
-#--#y#-M#-
-#y#---#y#-
-#---d---#-
-#--ddd--#-
-#M-----M#-
-####-####-
-#-#---#-#-
-#M-#-#--#-
-##D#D#D##-
-----------
", 11, 12);

const PRISON: (&str, i32, i32) = ("
------------
-####DD####-
-#d-#--#-##-
-#M-D--D-M#-
-##-#--#-##-
-####yy#d##-
-###d--####-
-####yy#d##-
-#M-D--D-M#-
-##d#--####-
-####DD####-
------------
", 12, 12);

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut structure = ("", 0, 0);
    let mut placement = None;
    let mut attempts = 0;
    println!("prefab.rs player_start: {:?}", mb.player_start);

    let d_map = DijkstraMap::new(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0
    );

    //let mut attempts = 0;
    for _i in 0 ..= 3 {
        while placement.is_none() && attempts < 10 {    //while placement is empty, and attempts are less than 10, loop
            structure = match rng.range(0, 9) { //randomly select a prefab structure to place
                0 => FORTRESS,
                1 => LABYRINTH,
                2 => MAZE,
                3 => TOMB,
                4 => FORTRESS_1,
                5 => LABYRINTH_1,
                6 => MAZE_1,
                7 => TOMB_1,
                _ => PRISON
            };
            let dimensions = Rect::with_size(   //create a Rect type starting at a random map location, with the height and width of the vault
                rng.range(0, SCREEN_WIDTH - structure.1),
                rng.range(0, SCREEN_HEIGHT - structure.2),
                structure.1,
                structure.2
            );

            let mut can_place = false;
            dimensions.for_each(|pt| {  //iterate every tile in the Rectangle via for_each
                let idx = mb.map.point2d_to_index(pt);
                let distance = d_map.map[idx];
                if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt && mb.player_start != pt {  //if D map distance for the tile is < 2000.0, the tile is reachable (ensure prefab does not contain amulet spawn or player_start)
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
            let mut row_num = 0i32;
            let mut col_num = 0i32;
            let string_vec: Vec<char> = structure.0
                .chars().filter(|a| *a != '\r' && *a != '\n')   //use an iterator to remove \r and \n charecters in the string literal
                .collect();
            let mut i = 0;  //represents the current location in the prefab as we iterate through it
            for ty in placement.y .. placement.y + structure.2 { //iterate every tile the prefab will cover
                //println!("current ty {}", ty);
                for tx in placement.x .. placement.x + structure.1 {
                    //println!("currrent tx {}", tx);
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
                        'd' => {
                            match rng.range(0, 2) {
                                0 => mb.map.tiles[idx] = TileType::Decorative,
                                _ => mb.map.tiles[idx] = TileType::OtherDecorative
                            }
                        },
                        'D' => mb.map.tiles[idx] = TileType::Door,
                        _ => println!("No idea what to do with {}", c)
                    }
                    //println!("row_n: {}, col_n: {}", row_num, col_num);
                    if (row_num > 0 && row_num < structure.2) && (col_num > 0 && col_num < structure.1) {
                        //println!("prefab_indices pushing {}", idx);
                        mb.map.prefab_indices.push(idx);    //log given space as being "in-prefab", but only if it is not in first/last row or column
                    }
                    //mb.map.prefab_indices.push(idx);    //log given space as being "in-prefab"
                    i += 1;
                    col_num += 1;   //increment the column number
                }
                col_num = 0;
                row_num += 1;   //increment the row number
            }
        }
    }
}