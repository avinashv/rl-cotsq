use crate::prelude::*;

#[system(for_each)]
pub fn fov(pos: &Point, fov: &mut FieldOfView, _ecs: &SubWorld, #[resource] map: &Map) {
    // Process a new fov calculation if the dirty flag is set
    if fov.is_dirty {
        fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
        fov.is_dirty = false; // unflag dirty
    }
}
