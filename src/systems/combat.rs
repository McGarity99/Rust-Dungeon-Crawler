use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(ProvidesScore)]
#[write_component(Score)]
#[write_component(Health)]
#[write_component(Armor)]
#[read_component(Damage)]
#[read_component(Carried)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let mut take_health = false;
    let mut health_damage = 0;

    /* let mut score_query = <&Score>::query().filter(component::<Player>());
    let mut player_score = score_query.iter(ecs).nth(0).unwrap(); */

    let victims: Vec<(Entity, Entity, Entity)> = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect();

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();
        let final_damage = base_damage + weapon_damage;
        if let Ok(mut armor) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Armor>() {
            if armor.current > 0 {
                println!("damage received: {}", final_damage);
                let new_damage = final_damage - armor.current; //calculate damage taken by armor
                if new_damage < 0 {
                    //if armor absorbs all damage with armor points left over
                    armor.current = new_damage * -1;
                } else if new_damage == 0 {
                    //if armor absorbs all damage with no armor points left over
                    armor.current = 0;
                } else {
                    //if damage is enough to "break" armor and damage player's health
                    take_health = true;
                    health_damage = new_damage;
                }
            } else {
                take_health = true; //move to take away health if armor is 0
                health_damage = final_damage;
            }
        } else {
            println!("No armor component for victim: {:?}", victim);
        }
        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            if is_player && take_health {
                //if player is attacked (and no armor or broken armor)
                health.current -= health_damage;
            } else if !is_player {
                //if monster is attacked (apply player's final_damage)
                health.current -= final_damage;
            }
            if health.current < 1 && !is_player {
                if let Ok(ProvidesScore) = ecs
                    .entry_mut(*victim)
                    .unwrap()
                    .get_component::<ProvidesScore>()
                {
                    let score_yield = ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<ProvidesScore>();
                    if let Ok(s_y) = score_yield {
                        let mut score_query = <&Score>::query().filter(component::<Player>());
                        let mut player_score = score_query.iter(ecs).nth(0).unwrap();
                        player_score.current = i32::min(player_score.max, player_score.current + s_y.amount);
                    }
                    /* match score_yield {
                        Ok(s_y) => {
                            player_score.current += i32::min(player_score.max, player_score.current + s_y.amount);
                        },
                        _ => {}
                    } */
                }
                commands.remove(*victim);
            }
        }

        /* if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {

            println!("combat.rs Health before attack: {}", health.current);
            health.current -= 1;
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
            println!("combat.rs Health after attack: {}", health.current)
        } */
        commands.remove(*message);
    })
}
