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
    pub use std::fs::File;
    pub use std::fs::OpenOptions;
    pub use std::io::BufReader;
    pub use std::io::Read;
    pub use std::io::Write;
    pub use std::thread;
    pub use rodio::{Decoder, OutputStream, Sink};
    pub use rodio::source::{SineWave, Source};
    pub use std::path::Path;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const REG_FOV: i32 = 8; //represents the player's FOV starting radius
    pub const FOV_REDUC: i32 = 1;   //represents amount by which player's FOV radius is reduced by special enemies
    pub const SCORE_STEAL_AMT: i32 = 100;
    pub const MAX_LEVEL: u32 = 3;
    pub const START_LEVEL: u32 = 0; //for debugging purposes (controls the theme the player starts in [0 Forest, 1 Dungeon, 2 Temple, 3 Volcano])
    pub const POISON_DMG: i32 = 1;  //const representing damage dealt by poison floors
    pub const START_P_RESISTANCE: i32 = 2;  //amount of poison resistance the player starts the game with
    pub const MAX_P_RESISTANCE: i32 = 5;    //max poison resistance the player can have
    pub const HEALTH_WARN_THRESHOLD: i32 = 5;   //threshold below which the player will get an audio warning of low health
    pub const SCORES_LOC: &str = "../scores/scores.txt";    //path of the scores file, change if needed
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
    monster_systems: Schedule,
    player_dead: bool
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, START_LEVEL as i32);
        let player_final_score = 0i32;
        let  score_message = String::new();
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.tome_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        spawn_level(
            &mut ecs,
            &mut rng,
            START_LEVEL as usize,
            &map_builder.monster_spawns,
            &mut map_builder.map
        );
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::Intro);
        resources.insert(map_builder.theme);
        resources.insert(player_final_score);
        resources.insert(score_message);
        
        Self { 
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            player_dead: false
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        let player_score: i32 = match self.resources.get::<i32>() {
            Some(num) => *num,
            None => 0
        };
        let score_line = format!("Final Score: {}", player_score);
        let score_message: String = match self.resources.get::<String>() {
            Some(msg) => msg.to_string(),
            None => String::new()
        };
        let high_score_line = format!("{}", score_message); 
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(4, WHITE, BLACK,
            "Slain by a beast, your hero's journey has come to a premature end, and your people have no hope.");
        ctx.print_color_centered(6, WHITE, BLACK,
            "The Tome of Anthrophulos remains unclaimed, and the city of Mharnem is consumed.");
        
        ctx.print_color_centered(8, WHITE, BLACK,
            "Don't worry, you can always try again with a new hero.");
        ctx.print_color_centered(10, YELLOW, BLACK,
            score_line.as_str());
        ctx.print_color_centered(11, YELLOW, BLACK,
            high_score_line.as_str());
        ctx.print_color_centered(13, GREEN, BLACK,
            "Press 1 to play again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        let player_score: i32 = match self.resources.get::<i32>() {
            Some(num) => *num,
            None => 0
        };
        let score_line = format!("Final Score: {:?}", player_score);
        let score_message: String = match self.resources.get::<String>() {
            Some(msg) => msg.to_string(),
            None => String::new()
        };
        let high_score_line = format!("{}", score_message);
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(4, WHITE, BLACK, "You flip through the pages of this ancient tome and feel its power.");
        ctx.print_color_centered(6, WHITE, BLACK, "The god of the lower realms does not take kindly to outsiders, so you decide not to hang around.");
        ctx.print_color_centered(8, WHITE, BLACK, "You ecstatically hurry back the way you came, past lumbering and grotesque beasts, ancient ruins and archaic halls");
        ctx.print_color_centered(10, WHITE, BLACK, "that are best left forgotten.");
        ctx.print_color_centered(12, WHITE, BLACK, "When you reach the surface, you frantically run through the forest back in the direction of your");
        ctx.print_color_centered(14, WHITE, BLACK, "forsaken city, in the direction of the wails of your people.");
        ctx.print_color_centered(16, WHITE, BLACK, "Those monsters from below have completely overrun the streets of Mharnem.");
        ctx.print_color_centered(18, WHITE, BLACK, "You fight your way to the city's Temple of Anthrophulos where most of the survivors wait, fearing you dead.");
        ctx.print_color_centered(20, WHITE, BLACK, "Showing the Tome to the sole remaining priest, a ray of hope flashes in his eyes, and the eyes of all those present.");
        ctx.print_color_centered(22, WHITE, BLACK, "Skimming some of the Tome's pages, the priest begins to utter a strange incantation in an unfamiliar tongue, but you recognize some of the names:");
        ctx.print_color_centered(24, WHITE, BLACK, "Anthrophulos, Yenuma, Bathastu and Nodos. Suddenly, the abominable howls outside fall silent, and the townsfolk");
        ctx.print_color_centered(26, WHITE, BLACK, "peer out from the safety of the temple, to see that all the horrors have seemingly vanished, and a ray of light has broken through the dark clouds above.");
        
        ctx.print_color_centered(28, WHITE, BLACK, "Though not the first attack on the city of Mharnem, hopefully this is the last thanks to your bravery.");
        ctx.print_color_centered(30, WHITE, BLACK, "Hopefully...");
        ctx.print_color_centered(32, YELLOW, BLACK, score_line.as_str());
        ctx.print_color_centered(34, YELLOW, BLACK, high_score_line.as_str());
        ctx.print_color_centered(36, GREEN, BLACK, "Press 1 to play again");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }

    fn new_game(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, GRAY, "Sanctuary");
        ctx.print_color_centered(4, WHITE, BLACK, "Your home city of Mharnem is beset by unnatural creatures.");
        ctx.print_color_centered(6, WHITE, BLACK, "Your elders have prayed and sacrificed to the gods of earth, with no success.");
        ctx.print_color_centered(8, WHITE, BLACK, "Desperate to stop these horrors, the townsfolk choose you by lottery to venture forth.");
        ctx.print_color_centered(10, WHITE, BLACK, "You must brave the forsaken wood that surrounds the city, explore the desolate catacombs that lie beneath,");
        ctx.print_color_centered(12, WHITE, BLACK, "and brace yourself for what lies below that. You are told to find the Tome of Anthrophulos.");
        ctx.print_color_centered(14, WHITE, BLACK, "With this book of hidden knowledge written by the warden of man, hopefully you can save Mharnem and appease the gods of earth");
        ctx.print_color_centered(16, WHITE, BLACK, "before you, your city, and your people are little more than a memory.");
        ctx.print_color_centered(20, RED, BLACK, "CONTROLS:");
        ctx.print_color_centered(22, RED, BLACK, "Use Arrow Keys to move");
        ctx.print_color_centered(24, RED, BLACK, "Use 'G' to pickup items, [6, 7, 8, 9] to use items");
        ctx.print_color_centered(26, RED, BLACK, "Avoid enemies, or move into them for combat");
        ctx.print_color_centered(28, RED, BLACK, "Maintain your armor, and replenish it if needed");
        ctx.print_color_centered(30, RED, BLACK, "Hover mouse over items/enemies for tooltips");
        ctx.print_color_centered(32, RED, BLACK, "Poison floors will damage your health directly, so make sure you have some anti-poison handy");


        ctx.print_color_centered(34, WHITE, BLACK, "Press 1 to begin");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
    }   //give player intro screen

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        self.player_dead = false;
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng, 0);
        let player_final_score = 0i32;
        let score_message = String::new();
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.tome_start);
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
            self.resources.insert(player_final_score);
            self.resources.insert(score_message);
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
                spawn_tome_of_anth(&mut self.ecs, map_builder.tome_start);
            } else {
                let exit_idx = map_builder.map.point2d_to_index(map_builder.tome_start);
                map_builder.map.tiles[exit_idx] = TileType::Exit;
            }   //spawn tome or stairs depending on the map level

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
                self.player_dead = true;
                self.game_over(ctx);
            },
            TurnState::Victory => {
                self.victory(ctx);
            },
            TurnState::NextLevel => {
                self.advance_level();
            },
            TurnState::Intro => {
                self.new_game(ctx);
            }
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()   //create generic terminal and specify attributes directly
        .with_title("Sanctuary")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT) //specify the size of subsequent consoles you add
        .with_tile_dimensions(32, 32)   //tile dimensions specifies the size of each character/object in the font file
        .with_resource_path("../resources/")
        .with_font("DungeonSpriteSheet.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "DungeonSpriteSheet.png")  //add a console using the specified dimensions and the named tile graphics file
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "DungeonSpriteSheet.png")    //add a second console with no background so transparency shows through it
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
