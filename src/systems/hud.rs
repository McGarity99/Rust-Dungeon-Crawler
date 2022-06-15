use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Armor)]
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
    /* match player_health {
        Some(_) => {
            let p_h = player_health.unwrap();

            let mut draw_batch = DrawBatch::new();
            draw_batch.target(2);   //draw to hud console layer
            draw_batch.print_centered(1, "Explore the Catacombs. Cursor Keys to move.");    //greet the player
            draw_batch.bar_horizontal(
                Point::zero(),
                SCREEN_WIDTH * 2,
                p_h.current,  //current value of health bar
                p_h.max,      //max value of health bar (bracket-lib will scale for you)
                ColorPair::new(RED, BLACK)  //color of full, empty
            );  //draw health bar
            draw_batch.print_color_centered(
                0,
                format!("Health: {} / {} ",
                p_h.current,
                p_h.max
                ),
                ColorPair::new(WHITE, RED)
            );
            draw_batch.submit(10000).expect("Batch error");
        },
        _ => {}
    } */

        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);   //draw to hud console layer
        draw_batch.print_centered(0, "Explore the Catacombs. Cursor Keys to move.");    //greet the player
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
            ColorPair::new(YELLOW, BLACK)
        );

        let player = <(Entity, &Player)>::query()
            .iter(ecs)
            .find_map(|(entity, _player)| Some(*entity))
            .unwrap();
        let mut item_query = <(&Item, &Name, &Carried)>::query();
        let mut y = 9;
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