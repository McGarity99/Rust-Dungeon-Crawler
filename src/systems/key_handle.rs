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
            map.key_carried = false;
        }
    }   //if player has a key, set map attribute to allow them to pass through doors
}