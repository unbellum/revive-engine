use crate::{mesh::Mesh, texture::Texture};

pub struct WorldObject
{
    shapes: Vec<Mesh>,
    textures: Vec<Texture>,
}