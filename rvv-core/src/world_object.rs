use crate::{mesh::Mesh, texture::Texture};

pub struct WorldObject
{
    file: String,
    shapes: Vec<Mesh>,
    textures: Vec<Texture>,

    // TODO: Object's orientation in the world
    //pos: Vec3,
    //transform: Mat4x4,
}

impl WorldObject
{
    pub fn load_from_file(_file: String) -> bool
    {
        // TODO:
        //
        // 1. What file format do we want for our wold objects? JSON, toml, yml?
        // 2. Load child assets
        //  * Mesh - Static mesh, animation info
        //  * Texture - Color, bump map, cube map, channel info, metadata (clamping, filtering, etc.)
        //  * Script?
        true
    }
}