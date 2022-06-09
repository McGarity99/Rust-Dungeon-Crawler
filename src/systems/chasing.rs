use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(ChasingPlayer)]
#[read_component(FieldOfView)]
#[read_component(Health)]
#[read_component(Point)]
pub fn chasing(
    #[resource] map: &Map,
    ecs: &SubWorld,
    commands: &mut CommandBuffer
) {
    let mut movers = <(Entity, &Point, &ChasingPlayer, &FieldOfView)>::query();   //find entities with Point positions and ChasingPlayer tags
    let mut positions = <(Entity, &Point, &Health)>::query();   //find all entities with Point and Health components
    let mut player = <(&Point, &Player)>::query();  //find only entities with Point positions and Player tag

    let player_pos = player.iter(ecs).nth(0).unwrap().0;    //extract the first entry from player query (.0 indicates Point or player's position)
    let player_idx = map_idx(player_pos.x, player_pos.y);

    let search_targets = vec![player_idx];  //create vector containing the tile index of the player as a start point
    let dijkstra_map = DijkstraMap::new(
        SCREEN_WIDTH,   //first 2 params define map size, not derived from the map itself
        SCREEN_HEIGHT,
        &search_targets,
        map,    //map is already &Map, so don't need to borrow it again here
        1024.0  //D. map calculation for a large map is slow, pick a number large enough to cover most of the map, but small enough to not slow down the computer
    );

    movers.iter(ecs).for_each(|(entity, pos, _, fov)| {  //iterate over all entities with the ChasingPlayer tag
        if !fov.visible_tiles.contains(&player_pos) {   //ensure monsters only chase the player if they can see them
            return;
        }
        let idx = map_idx(pos.x, pos.y);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) { //f_l_e function finds the exit with the lowest distance to your target point
            let distance = DistanceAlg::Pythagoras.distance2d(*pos, *player_pos);   //use Pythagoras' algorithm to determine distance to the player
            let destination = if distance > 1.2 {   //if player more than 1.2 tiles away, set the destination to the result of the D. map search
                map.index_to_point2d(destination)
            } else {
                *player_pos
            };

            let mut attacked = false;
            positions
                .iter(ecs)
                .filter(|(_, target_pos, _)| **target_pos == destination)
                .for_each(|(victim, _, _)| {
                    if ecs.entry_ref(*victim).unwrap().get_component::<Player>().is_ok() {
                        commands
                            .push(((), WantsToAttack {
                                attacker: *entity,
                                victim: *victim
                            }));
                    }
                    attacked = true;
                });
            if !attacked {
                commands
                    .push(((), WantsToMove { entity: *entity, destination}));
            }
        }
    });
}