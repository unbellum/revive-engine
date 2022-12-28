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
    pub fn new() -> WorldObject
    {
        WorldObject {
            file: "".to_string(),
            shapes: None,
            textures: None
        }
    }

    pub fn load_from_file(self, _display: &glium::Display, _file: String) -> Self
    {
        // TODO:
        //
        // 1. What file format do we want for our wold objects? JSON, toml, yml?
        // 2. Load child assets
        //  * Mesh - Static mesh, animation info
        //  * Texture - Color, bump map, cube map, channel info, metadata (clamping, filtering, etc.)
        //  * Script?
        self
    }

    // Create a cube mesh object and add it to the list of shapes
    pub fn load_basic_cube(self, display: &glium::Display) -> Self
    {
        let cube: Mesh = Mesh::new().load_cube(display);
        self.shapes.push(cube);
        self
    }
}