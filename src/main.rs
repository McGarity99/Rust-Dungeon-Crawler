mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const REG_FOV: i32 = 8;
    pub const NV_FOV: i32 = 12;
    pub const SCORE_STEAL_AMT: i32 = 50;
    pub const MAX_LEVEL: u32 = 3;
    pub const START_LEVEL: u32 = 0; //for debugging purposes (controls the theme the player starts in [0 Forest, 1 Dungeon, 2 Temple, 3 Volcano])
    pub const POISON_DMG: i32 = 1;  //const representing damage dealt by poison floors
    pub const START_P_RESISTANCE: i32 = 2;  //amount of poison resistance the player starts the game with
    pub const MAX_P_RESISTANCE: i32 = 5;    //max poison resistance the player can have
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, START_LEVEL as i32);
        spawn_player(&mut ecs, map_builder.player_start);
        //spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        //println!("spawn points: {:?}", &map_builder.monster_spawns);
        spawn_level(
            &mut ecs,
            &mut rng,
            START_LEVEL as usize,
            &map_builder.monster_spawns,
            &mut map_builder.map
        );
        /* map_builder.rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_entity(&mut ecs, &mut rng, pos)); */
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);
        Self { 
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler()
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        println!("entering game over");
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(4, WHITE, BLACK,
            "Slain by a monster, your hero's journey has come to a premature end");
        ctx.print_color_centered(5, WHITE, BLACK,
            "The Tome of Anthrophulos remains unclaimed, and the city of Mharnem is consumed");
        
        ctx.print_color_centered(8, YELLOW, BLACK,
            "Don't worry, you can always try again with a new hero");
        ctx.print_color_centered(9, GREEN, BLACK,
            "Press 1 to play again");
        
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK, "You put on the Amulet of Yala and feel its power");
        ctx.print_color_centered(5, WHITE, BLACK, "Mharnem is saved, and you can return to your normal life");
        ctx.print_color_centered(7, GREEN, BLACK, "Press 1 to play again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, 0);
        spawn_player(&mut self.ecs, map_builder.player_start);
        //spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(
            &mut self.ecs,
            &mut rng,
            START_LEVEL as usize,
            &map_builder.monster_spawns,
            &mut map_builder.map
        );
            self.resources.insert(map_builder.map);
            self.resources.insert(Camera::new(map_builder.player_start));
            self.resources.insert(TurnState::AwaitingInput);
            self.resources.insert(map_builder.theme);
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();

        let mut level_id: i32 = 0;
        if let Ok(p_score) = self.ecs.entry_mut(player_entity).unwrap().get_component_mut::<Score>() {
            p_score.level_theme += 1;
            level_id = p_score.level_theme;
        }   //advance level id token to spawn correct level theme
            use std::collections::HashSet;
            let mut entities_to_keep = HashSet::new();
            entities_to_keep.insert(player_entity); //we want to keep the player obviously

            <(Entity, &Carried)>::query()
                .iter(&self.ecs)
                .filter(|(_e, carry)| carry.0 == player_entity)
                .map(|(e, _carry)| *e)
                .for_each(|e| {entities_to_keep.insert(e);});   //add the entity to the keep HashSet if they are being carried by the player
            
            let mut cb = CommandBuffer::new(&mut self.ecs);
            for e in Entity::query().iter(&self.ecs) {
                if !entities_to_keep.contains(e) {
                    cb.remove(*e);
                }
            }
            cb.flush(&mut self.ecs);

            <&mut FieldOfView>::query()
                .iter_mut(&mut self.ecs)
                .for_each(|fov| fov.is_dirty = true);   //flag the player's FOV as dirty so that it doesn't carry over to next dungeon level

            let mut rng = RandomNumberGenerator::new();
            let mut map_builder = MapBuilder::new(&mut rng, level_id);

            let mut map_level = 0;
            <(&mut Player, &mut Point)>::query()
                .iter_mut(&mut self.ecs)
                .for_each(|(player, pos)| {
                    player.map_level += 1;
                    map_level = player.map_level;
                    pos.x = map_builder.player_start.x;
                    pos.y = map_builder.player_start.y;
                }
            );

            if map_level == MAX_LEVEL {
                spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
            } else {
                let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
                map_builder.map.tiles[exit_idx] = TileType::Exit;
                println!("exit loc: {:?}", map_builder.amulet_start);
            }   //spawn amulet for stairs depending on the map level

            spawn_level(&mut self.ecs, &mut rng, map_level as usize, &map_builder.monster_spawns, &mut map_builder.map);
            self.resources.insert(map_builder.map);
            self.resources.insert(Camera::new(map_builder.player_start));
            self.resources.insert(TurnState::AwaitingInput);
            self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self.input_systems.execute(
                &mut self.ecs,
                &mut self.resources
            ),
            TurnState::PlayerTurn => self.player_systems.execute(
                &mut self.ecs,
                &mut self.resources
            ),
            TurnState::MonsterTurn => self.monster_systems.execute(
                &mut self.ecs, &mut self.resources
            ),
            TurnState::GameOver => {
                self.game_over(ctx);
            },
            TurnState::Victory => {
                self.victory(ctx);
            },
            TurnState::NextLevel => {
                self.advance_level();
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()   //create generic terminal and specify attributes directly
        .with_title("Catacomb Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //specify the size of subsequent consoles you add
        .with_tile_dimensions(32, 32)   //tile dimensions specifies the size of each character/object in the font file
        .with_resource_path("../resources/")
        .with_font("DungeonSpriteSheet.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "DungeonSpriteSheet.png")  //add a console using the specified dimensions and the named tile graphics file
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "DungeonSpriteSheet.png")    //add a second console with no background so transparency shows through it
        //.with_sprite_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, 32)
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
