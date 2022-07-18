use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Utility)]
pub fn key_handle(
    ecs: &mut SubWorld,
    #[resource] map: &mut Map
) {

    let mut player_query = <Entity>::query().filter(component::<Player>());
    let player = player_query.iter(ecs).nth(0).unwrap();

    let mut key_query = <(&Item, &Carried, &Utility)>::query();
    match key_query.iter(ecs).filter(|(_, carried, _) | carried.0 == *player).nth(0) {
        Some(_) => {
            map.key_carried = true;
        },
        None => {
            let mut player_point_q = <&Point>::query().filter(component::<Player>());
            let player_point = player_point_q.iter(ecs).nth(0).unwrap();
            let position = map.point2d_to_index(*player_point); //represents player's position as a usize
            if !map.prefab_indices.contains(&position) {    //if player is not currently inside a prefab structure,
                map.key_carried = false;                    //then mark player as having no key, preventing them from locking themselves inside a prefab
            }
        }
    }   //if player has a key, set map attribute to allow them to pass through doors
}