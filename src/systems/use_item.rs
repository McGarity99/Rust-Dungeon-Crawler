use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[write_component(FieldOfView)]
#[read_component(ProvidesNVision)]
#[read_component(Player)]
pub  fn use_item(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] map: &mut Map
) {
    /* let mut fov_query = <&mut FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov_query.iter_mut(ecs).nth(0).unwrap();
    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate)| {
        let item = ecs.entry_ref(activate.item);
        if let Ok(item) = item {
            if let Ok(nv) = item.get_component::<ProvidesNVision>() {
                //let new_fov = player_fov.clone_dirty();
                println!("got Night Vision");
            } else {
                println!("not Night Vision");
            }
        }
    }); */
    /* let mut player_query = <&mut Player>::query().iter(ecs).nth(0).unwrap();
    let player_fov = fov_query.iter_mut(ecs).nth(0).unwrap();
    player_fov.set_fov(); */

    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate)| {
        let item = ecs.entry_ref(activate.item);
        if let Ok(item) = item {
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                healing_to_apply.push((activate.used_by, healing.amount));  //if ProvidesHealing component is present, push healing amount to the vec
            }
            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);  //if ProvidesDungeonMap component is present, reveal all map tiles
            } else {
                println!("not NV");
            }
        }
        commands.remove(activate.item);
        commands.remove(*entity);
    });
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1);
            }
        }
    }
}