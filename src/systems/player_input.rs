use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[write_component(Score)]
#[read_component(ScoreItem)]
#[read_component(ProvidesScore)]
#[read_component(Enemy)]
#[read_component(Health)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Weapon)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    if let Some(key) = key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>()); //only entities with a Point component and a Player tag component should be included in the query

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let mut picked_up_gold = false; //flag for whether the player picked up gold or some other item
                let mut score_amt = 0i32;   //temp variable to store the amount added to the player's score from ScoreItem

                //let mut score_to_apply = Vec::<(Entity, i32)>::new();

                let (_player, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                
                let mut player_query = <Entity>::query().filter(component::<Player>()); //get the score (only the player has a Score component)
                let mut player_entity = player_query.iter(ecs).nth(0).unwrap(); //get player so we can access their Score component

                let mut gold_locs = <(Entity, &ScoreItem, &Point, &ProvidesScore)>::query();   //get all entities that have a ScoreItem & Point component
                gold_locs
                    .iter(ecs)
                    .filter(|(_entity, s_i, &pos, &p_s)| pos == player_pos)
                    .for_each(|(entity, s_item, position, provides_score)| {
                        println!("gold at pos: {:?}", position);
                        picked_up_gold = true;
                        score_amt += provides_score.amount;
                        commands.remove_component::<Point>(*entity);
                    }); //iterate over all found entities, filter out all that are not at the same position as the player
                        //for each matching entity (max is 1), set the flag to true and remove it from the game world

                if picked_up_gold {
                    if let Ok(mut score) = ecs.entry_mut(*player_entity)
                        .unwrap()
                        .get_component_mut::<Score>()
                    {
                        score.current += score_amt;
                    }
                } else {
                    /* gold_locs.iter(ecs)
                        .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                        .for_each(|(entity, item, _item_pos)| {
                            println!("got gold");
                            picked_up_gold = true;
                        }
                    ); */

                    let mut temp_count = 0;
                    let mut temp_carried_query = <&Carried>::query();
                    temp_carried_query.iter(ecs).for_each(|_c| {
                        temp_count += 1;
                    });

                    println!("carried items: {}", temp_count);

                    if temp_count < 3 {
                        println!("less than 3, carrying item");
                        let (player, player_pos) = players
                            .iter(ecs)
                            .find_map(|(entity, pos)| Some((*entity, *pos)))
                            .unwrap();

                        let mut items = <(Entity, &Item, &Point)>::query();

                        items
                            .iter(ecs)
                            .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)
                            .for_each(|(entity, _item, _item_pos)| {
                                println!("adding Carried(player)");
                                commands.remove_component::<Point>(*entity);
                                commands.add_component(*entity, Carried(player));

                                if let Ok(e) = ecs.entry_ref(*entity) {
                                    if e.get_component::<Weapon>().is_ok() {
                                        <(Entity, &Carried, &Weapon)>::query()
                                            .iter(ecs)
                                            .filter(|(_, c, _)| c.0 == player)
                                            .for_each(|(e, _c, _w)| {
                                                commands.remove(*e);
                                            })
                                    }
                                }
                            });
                        //Point::new(0, 0)
                    }
                }
                Point::new(0, 0)
            }
            /* VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands), */
            VirtualKeyCode::Key7 => {
                //println!("using_item 6, ecs, commands");
                use_item(0, ecs, commands)
            }
            VirtualKeyCode::Key8 => {
                //println!("using_item 7, ecs, commands");
                use_item(1, ecs, commands)
            }
            VirtualKeyCode::Key9 => {
                //println!("using_item 8, ecs, commands");
                use_item(2, ecs, commands)
            }
            _ => Point::new(0, 0),
        };

        let mut did_something = false;

        if delta.x != 0 || delta.y != 0 || !did_something {
            /* let mut players = <(Entity, &Point)>::query()
            .filter(component::<Player>()); //only entities with a Point component and a Player tag component should be included in the query */

            /* match players.iter(ecs).nth(0) {
                Some(_) => {},
                None => return,
            } */

            let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
                .unwrap();

            let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;
                    did_something = true;
                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                did_something = true;
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }

            //*turn_state = TurnState::PlayerTurn;
        }

        *turn_state = TurnState::PlayerTurn;
    }
}

fn use_item(n: usize, ecs: &mut SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _player)| Some(*entity))
        .unwrap(); //query to find the player entity

    let item_entity = <(Entity, &Item, &Carried)>::query()
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _, _))| *item_count == n) //index n corresponds to the number pressed by the player
        .find_map(|(_, (item_entity, _, _))| Some(*item_entity)); //iterate through carried items and filter out those not carried by the player

    if let Some(item_entity) = item_entity {
        //need if-let here because find_map could return None
        //println!("commands pushing... ");
        commands.push((
            (),
            ActivateItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }
    Point::zero()
}
