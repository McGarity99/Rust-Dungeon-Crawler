use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn tooltips(ecs: &SubWorld, 
    #[resource] mouse_pos: &Point, //read-only access to the mouse position inserted as a point in main.rs
    #[resource] camera: &Camera //read-only access to the camera
) {
    let mut positions = <(Entity, &Point, &Name)>::query(); //returns the Entity and the Point & Name components from entites that have both of these components
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;  //current position plus the left and top of the screen gives the screen position of an entity

    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(&pos));

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    positions
        .iter(ecs)
        .filter(|(_, pos, _)| **pos == map_pos) //only include elements whose Point position is equal to the current mouse cursor position stored in map_pos
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 4;    //mouse position is in coordinates aligning with the monster layer; tooltips layer is 4 times larger
            let display = if let Ok(health) = ecs.entry_ref(*entity)    //use entry_ref to access an entity's components from outside a query
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current)
            } else {    //if hovering over a non-enemy entity (ex: treasure), just display its name
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}