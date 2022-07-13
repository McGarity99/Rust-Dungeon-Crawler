use crate::prelude::*;

#[system]
#[read_component(ActivateItem)]
#[read_component(ProvidesHealing)]
#[write_component(Health)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Armor)]
#[read_component(ProvidesArmor)]
#[write_component(FieldOfView)]
#[read_component(ProvidesNVision)]
#[read_component(ProvidesPoisonR)]
#[write_component(Score)]
#[read_component(Player)]
pub fn use_item(
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
    let mut armor_to_apply = Vec::<(Entity, i32)>::new();
    let mut poison_res_to_apply = Vec::<(Entity, i32)>::new();
    <(Entity, &ActivateItem)>::query().iter(ecs)
    .for_each(|(entity, activate)| {
        //println!("activate: {:?}", activate);
        let item = ecs.entry_ref(activate.item);
        if let Ok(item) = item {
            if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                healing_to_apply.push((activate.used_by, healing.amount));  //if ProvidesHealing component is present, push healing amount to the vec
            }
            if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                map.revealed_tiles.iter_mut().for_each(|t| *t = true);  //if ProvidesDungeonMap component is present, reveal all map tiles
            }
            if let Ok(armor) = item.get_component::<ProvidesArmor>() {
                println!("armor to apply pushing: {}", armor.amount);
                armor_to_apply.push((activate.used_by, armor.amount));
            }
            if let Ok(poison_res) = item.get_component::<ProvidesPoisonR>() {
                poison_res_to_apply.push((activate.used_by, poison_res.amount));
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
    }   //apply healing to the target, taking min between max and current + amount, to avoid going over the maximum

    for armor in armor_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(armor.0) {
            if let Ok(armor_amt) = target.get_component_mut::<Armor>() {
                armor_amt.current = i32::min(armor_amt.max, armor_amt.current + armor.1);
            }
        }
    }   //apply armor to the target, taking min between max and current + amount, to avoid going over the maximum

    for p_r in poison_res_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(p_r.0) {
            if let Ok(score) = target.get_component_mut::<Score>() {
                score.poison_shield = i32::min(score.max_poison_shield, score.poison_shield + p_r.1);
            }
        }
    }
}