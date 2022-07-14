use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Armor)]
#[read_component(Score)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap();
    let mut armor_query = <&Armor>::query().filter(component::<Player>());
    let player_armor = armor_query
        .iter(ecs)
        .nth(0)
        .unwrap();
    let mut score_query = <&Score>::query().filter(component::<Player>());
    let player_score = score_query
        .iter(ecs)
        .nth(0)
        .unwrap();

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);   //draw to hud console layer
        draw_batch.print_centered(0, "Explore the Environment. Cursor Keys to move.");    //greet the player
        draw_batch.bar_horizontal(
            Point::new(2, 2),
            SCREEN_WIDTH,
            player_health.current,  //current value of health bar
            player_health.max,      //max value of health bar (bracket-lib will scale for you)
            ColorPair::new(RED, BLACK)  //color of full, empty
        );  //draw health bar
        draw_batch.print_color(
            Point::new(0, 2),
            format!("Health:\t{} / {} ",
            player_health.current,
            player_health.max
            ),
            ColorPair::new(WHITE, RED)
        );
        draw_batch.bar_horizontal(
            Point::new(2, 4),
            SCREEN_WIDTH,
            player_armor.current,
            player_armor.max,
            ColorPair::new(GREEN, BLACK)
        );
        draw_batch.print_color(
            Point::new(0, 4),
            format!("Armor:\t{} / {} ",
            player_armor.current,
            player_armor.max
            ),
            ColorPair::new(WHITE, GREEN)
        );

        let (player, map_level) = <(Entity, &Player)>::query()
            .iter(ecs)
            .find_map(|(entity, player)| Some((*entity, player.map_level)))
            .unwrap();
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH*2, 1),
            format!("Dungeon Level: {}", map_level+1),
            ColorPair::new(BLUE, BLACK)
        );

        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH * 2, 3),
            format!("Score: {}", player_score.current),
            ColorPair::new(YELLOW, BLACK)
        );
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH * 2, 4),
            format!("Poison Resistance: {}", player_score.poison_shield),
            ColorPair::new(GHOSTWHITE, BLACK)
        );

        let level_name = match player_score.level_theme {
            0 => "Forgotten Forest",
            1 => "Decrepit Dungeon",
            2 => "Forsaken Temple",
            _ => "Netherworld"
        };
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH * 2, 5),
            format!("{}", level_name),
            ColorPair::new(RED, GREY)
        );

        let player = <(Entity, &Player)>::query()
            .iter(ecs)
            .find_map(|(entity, _player)| Some(*entity))
            .unwrap();
        let mut item_query = <(&Item, &Name, &Carried)>::query();
        let mut y = 8;
        item_query
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == player)
            .for_each(|(_, name, _)| {
                draw_batch.print(
                    Point::new(3, y),
                    format!("{}: {}", y-2, &name.0)
                );
                y += 1;
            }
        );

        if y > 3 {
            draw_batch.print_color(Point::new(3, 7), "~ Items carried ~", ColorPair::new(YELLOW, BLACK));
        }

        draw_batch.submit(10000).expect("Batch error");
    
}