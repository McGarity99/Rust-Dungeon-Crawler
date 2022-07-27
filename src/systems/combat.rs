use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[read_component(ProvidesScore)]
#[read_component(IgnoresArmor)]
#[read_component(StealsScore)]
#[read_component(ReducesFOV)]
#[write_component(Score)]
#[write_component(Health)]
#[write_component(Armor)]
#[read_component(Damage)]
#[read_component(Carried)]
#[write_component(FieldOfView)]
#[read_component(SmallMonster)]
#[read_component(LargeMonster)]
#[read_component(AllSeeing)]
#[read_component(StealsScore)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();
    let mut take_health = false;
    let mut health_damage = 0;
    let mut rng = RandomNumberGenerator::new();

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
        let is_visage: bool = ecs.entry_mut(*attacker).unwrap().get_component::<StealsScore>().is_ok();
        let is_angel: bool = ecs.entry_mut(*attacker).unwrap().get_component::<IgnoresArmor>().is_ok();
        let is_wraith: bool = ecs.entry_mut(*attacker).unwrap().get_component::<ReducesFOV>().is_ok(); 
        if let Ok(mut armor) = ecs.entry_mut(*victim).unwrap().get_component_mut::<Armor>() {
            if is_angel {   //if being attacked by Fallen Angel (ignores armor)
                take_health = true;
                health_damage = final_damage;
            } else {    //attacked by any other enemy type
                if armor.current > 0 {
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
                        armor.current = 0;
                        health_damage = new_damage;
                    }
                } else {
                    take_health = true; //move to take away health if armor is 0
                    health_damage = final_damage;
                }
            }
        }
        if let Ok(mut health) = ecs.clone()
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            if is_player && take_health {
                //if player is attacked (and no armor or broken armor)
                health.current -= health_damage;
                thread::spawn(|| {
                    let(_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    let file = BufReader::new(File::open("../resources/Stab_Knife_01.wav").unwrap());
                    let source = Decoder::new(file).unwrap();
                    sink.append(source);
                    sink.sleep_until_end();
                });

                if health.current <= HEALTH_WARN_THRESHOLD {
                    thread::spawn(|| {
                        let(_stream, stream_handle) = OutputStream::try_default().unwrap();
                        let sink = Sink::try_new(&stream_handle).unwrap();
                        let file = BufReader::new(File::open("../resources/Breath_Scared_17.wav").unwrap());
                        let source = Decoder::new(file).unwrap();
                        sink.append(source);
                        sink.sleep_until_end();
                    });
                }

                if is_visage {
                    if let Ok(score_steal) = ecs.clone().entry_mut(*attacker).unwrap().get_component::<StealsScore>() {
                        if let Ok(mut p_score) = ecs.clone().entry_mut(*victim).unwrap().get_component_mut::<Score>() {
                            p_score.current = i32::max(0, p_score.current - score_steal.amount);
                            thread::spawn(|| {
                                let(_stream, stream_handle) = OutputStream::try_default().unwrap();
                                let sink = Sink::try_new(&stream_handle).unwrap();
                                let file = BufReader::new(File::open("../resources/Score_Steal.wav").unwrap());
                                let source = Decoder::new(file).unwrap();
                                sink.append(source);
                                sink.sleep_until_end();
                            }); //if player's score is deducted by Visage, play the score steal sound
                        }   //get player's score and reduce it when fighting a Visage (with no armor)
                    }
                }
                if is_wraith {
                    if let Ok(fov) = ecs.clone().entry_mut(*victim).unwrap().get_component_mut::<FieldOfView>() {
                        if fov.radius > 5 {
                            fov.dec_fov();
                            thread::spawn(|| {
                                let(_stream, stream_handle) = OutputStream::try_default().unwrap();
                                let sink = Sink::try_new(&stream_handle).unwrap();
                                let file = BufReader::new(File::open("../resources/FOV_Down.wav").unwrap());
                                let source = Decoder::new(file).unwrap();
                                sink.append(source);
                                sink.sleep_until_end();
                            });
                        }   //do not allow player's FOV to slide below 4 tiles when fighting a Wraith (with no armor)
                    }
                }
            } else if !is_player {
                //if monster is attacked (apply player's final_damage)
                health.current -= final_damage;
            }
            if health.current < 1 && !is_player {
                if let Ok(_provides_score) = ecs
                    .entry_mut(*victim)
                    .unwrap()
                    .get_component::<ProvidesScore>()
                {
                    let _score_yield = if let Ok(score) = ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<ProvidesScore>() {
                            let mut player_query = <Entity>::query().filter(component::<Player>()); //query to get Entities with Player tag
                            let player_entity = player_query.iter(ecs).nth(0).unwrap(); //get player entity
                            if let Ok(mut p_score) = ecs.clone().entry_mut(*player_entity) //get mutable access to Player's score by cloning the SubWorld
                                .unwrap()
                                .get_component_mut::<Score>()
                            {
                                p_score.current = i32::min(p_score.max, p_score.current + score.amount);    //add to Player's score without going over the limit
                            }
                        };
                }

                if let Ok(_reduces_fov) = ecs.entry_mut(*victim).unwrap().get_component::<ReducesFOV>() {
                    play_sound(String::from("../resources/Wraith_Death.wav"));
                } else if let Ok(_all_seeing) = ecs.entry_mut(*victim).unwrap().get_component::<AllSeeing>() {
                    match rng.range(0, 2) {
                        0 => {
                            play_sound(String::from("../resources/Okulos_Death_1.ogg"));
                        },
                        _ => {
                            play_sound(String::from("../resources/Okulos_Death_2.ogg"));
                        }
                    }
                } else if let Ok(_ignores_armor) = ecs.entry_mut(*victim).unwrap().get_component::<IgnoresArmor>() {
                    play_sound(String::from("../resources/Angel_Death.wav"));
                } else if let Ok(_steals) = ecs.entry_mut(*victim).unwrap().get_component::<StealsScore>() {
                    play_sound(String::from("../resources/Visage_Death.wav"));
                } else if let Ok(_small_monster) = ecs.entry_mut(*victim).unwrap().get_component::<SmallMonster>() {
                    match rng.range(0, 3) {
                        0 => {
                            play_sound(String::from("../resources/Small_Monster_1.wav"));
                        },
                        1 => {
                            play_sound(String::from("../resources/Small_Monster_2.wav"));
                        },
                        _ => {
                            play_sound(String::from("../resources/Small_Monster_3.wav"));
                        }
                    }
                } else if let Ok(_large_monster) = ecs.entry_mut(*victim).unwrap().get_component::<LargeMonster>() {
                    match rng.range(0, 3) {
                        0 => {
                            play_sound(String::from("../resources/Large_Monster_1.wav"));
                        },
                        1 => {
                            play_sound(String::from("../resources/Large_Monster_2.wav"));
                        },
                        _ => {
                            play_sound(String::from("../resources/Large_Monster_3.wav"));
                        }
                    }
                }
                commands.remove(*victim);   //enemy is slain, so remove it from the game
            }
        }
        commands.remove(*message);
    })
}

/*
    This function is a helper function meant to handle the thread-spawning and sound-playing
    for enemy death sounds, thus decreasing the bulk of the combat system function above.
*/
fn play_sound(path: String) {
    thread::spawn(move || {
        let(_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file = BufReader::new(File::open(path.as_str()).unwrap());
        let source = Decoder::new(file).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    });
}
