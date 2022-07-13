use crate::prelude::*;

#[system(for_each)] //run the system once for every matching entity in the query
#[read_component(Player)]
#[read_component(FieldOfView)]
#[write_component(Health)]
#[write_component(Score)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    //let fov = <&FieldOfView>::query();

    if let Ok(entry) = ecs.entry_ref(want_move.entity) {
        if let Ok(fov) = entry.get_component::<FieldOfView>() {
            commands.add_component(want_move.entity, fov.clone_dirty());

            if entry.get_component::<Player>().is_ok() {
                camera.on_player_move(want_move.destination);
                fov.visible_tiles.iter().for_each(|pos| {
                    map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                });
            }
        }
        
    }

    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs.entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Player>().is_ok()
        {
            camera.on_player_move(want_move.destination);
            let temp_idx = map.point2d_to_index(want_move.destination);
            match map.tiles[temp_idx] {
                TileType::PoisonFloor => {
                    if let Ok(mut health) = ecs.clone().entry_mut(want_move.entity).unwrap().get_component_mut::<Health>() {
                        if let Ok(mut score) = ecs.clone().entry_mut(want_move.entity).unwrap().get_component_mut::<Score>() {
                            match score.poison_shield {
                                0 => {
                                    health.current = i32::max(0, health.current - POISON_DMG);
                                },  //if no poison shield currently, subtract from health
                                _ => {
                                    score.poison_shield = i32::max(0, score.poison_shield - POISON_DMG);
                                }   //otherwise, subtract from poison shield
                            }
                        }   //get player's Score component (contains PoisonShield field)
                    }   //get player's Health component
                },
                _ => {}
            }   //identify PoisonFloor space as entered by the player
        }
    }
    commands.remove(*entity);
}