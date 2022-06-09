use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &Map
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();  //query that reads a Point position and can write to the FOV component
    views
        .iter_mut(ecs)
        .filter(|(_, fov)| fov.is_dirty)    //ensure that only dirty entries are updated
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);   //calculated field of view for a map that supports Algorithm2D, returns HashSet
            fov.is_dirty = false;   //mark field of view as not dirty - will not be updated again until it is flagged as dirty
        })
}