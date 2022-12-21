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

impl Scene
{
    pub fn load_from_file(_file: String) -> bool
    {
        // TODO: What file format do we want to support? Should be same as used for other assets
        true
    }
}