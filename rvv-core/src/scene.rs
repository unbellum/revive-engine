// What types of objects would we want in our scene to start?
//
// 1. Static world representation
// 2. Static objects
// 3. Player
// 4. Triggers
use crate::{world_object::WorldObject};

pub struct Scene
{
    objects: Vec<WorldObject>,
}