use crate::prelude::*;
use serde::Deserialize;
use ron::de::from_reader;
use std::fs::File;
use std::collections::HashSet;
use legion::systems::CommandBuffer;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub glyph: char,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy, Item, Score
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("../resources/template.ron")
            .expect("Failed opening file");
        from_reader(file).expect("Unable to load template")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point]
    ) {
        let mut available_entities = Vec::new();    //create vector in which we'll store a list of entities that can be spawned on this level
        self.entities
            .iter()     //iterate the list of entity templates loaded in the constructor
            .filter(|e| e.levels.contains(&level))  //filter out entities that cannot be spawned on this level
            .for_each(|t| {
                for _ in 0 .. t.frequency { //add each available entity many times equal to the entry's frequency field
                    available_entities.push(t);
                }
            }
        );

        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, &mut commands);
            }
        });
        commands.flush(ecs);
    }

    fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer
    ) {
       /*  println!("spawning: {}", template.name.clone());
        println!("at : {:?}", pt.clone()); */
        let entity = commands.push((    //push tuple of components to define the new entity
            pt.clone(), //clone the position from pt to use it as a new component
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437(template.glyph) //use the glyph value from the template
            },
            Name(template.name.clone()) //clone the name from the template
        ));

        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item{}),
            EntityType::Score => commands.add_component(entity, ScoreItem{}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy{});
                commands.add_component(entity, FieldOfView::new(6));
                commands.add_component(entity, ChasingPlayer{});
                commands.add_component(entity, Health {
                    current: template.hp.unwrap(),
                    max: template.hp.unwrap()
                });
            }
        }

        if let Some(effects) = &template.provides {
            effects.iter().for_each(|(provides, n)| {
                match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing{amount: *n}),
                    "MagicMap" => commands.add_component(entity, ProvidesDungeonMap{}),
                    "NVision" => commands.add_component(entity, ProvidesNVision{}),
                    "Armor" => commands.add_component(entity, ProvidesArmor{amount: *n}),
                    "Score" => commands.add_component(entity, ProvidesScore{amount: *n}),
                    _ => println!("Warning: we don't know how to provide {}", provides),
                }
            })
        }

        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon{});
            }
        }
    }
}