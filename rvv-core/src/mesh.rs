use crate::{texture::Texture};

#[derive(Copy,Clone)]
pub struct VBOPos
{
    position: [f32; 3],
}

#[derive(Copy,Clone)]
pub struct VBOPosNorm
{
    position: [f32; 3],
    normal: [f32; 3],
}

#[derive(Copy,Clone)]
pub struct VBOPosNormTex
{
    position: [f32; 3],
    normal: [f32; 3],
    texture: [f32; 2],
}

pub enum VertexBuffer
{
    Pos{
        vbo: Option<glium::VertexBuffer<VBOPos> >
    },
    PosNorm{
        vbo: Option<glium::VertexBuffer<VBOPosNorm> >
    },
    PosNormTex{
        vbo: Option<glium::VertexBuffer<VBOPosNormTex> >
    },

}

/// Handles the rendering of a single surface.
pub struct Surface
{
    vbo: VertexBuffer,
    ibo: Option<glium::IndexBuffer<u32> >,
    textures: Vec<Texture>,
}

impl Surface
{
    pub fn new() -> Surface
    {
        Surface {
            vbo: VertexBuffer::Pos { vbo: None },
            ibo: None,
            textures: Vec::new(),
        }
    }
    
    pub fn render(&self, target: &mut glium::Frame, program: &glium::Program)
    {
        use glium::{Surface};
        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ];
        // Wanted to create a generic function to handle some of the repetition but it does like
        // mixing generics with the enums for the surface format. Ideally would like a small function
        // to take in the vbo as Option<glium::VertexBuffer<T> > as the rest of the code is the same.
        //
        // Will a macro work better here?
        match &self.vbo
        {
            VertexBuffer::Pos { vbo } =>
            {
                match vbo
                {
                    Some(vbo_bound) =>
                    {
                        match &self.ibo
                        {
                            Some(ibo_bound) =>
                            {
                                target.draw(vbo_bound, ibo_bound, &program, &uniform! { matrix: matrix },
                                    &Default::default()).unwrap();
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            VertexBuffer::PosNorm { vbo } =>
            {
                match vbo
                {
                    Some(vbo_bound) =>
                    {
                        match &self.ibo
                        {
                            Some(ibo_bound) =>
                            {
                                target.draw(vbo_bound, ibo_bound, &program, &uniform! { matrix: matrix },
                                    &Default::default()).unwrap();
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            VertexBuffer::PosNormTex { vbo } =>
            {
                match vbo
                {
                    Some(vbo_bound) =>
                    {
                        match &self.ibo
                        {
                            Some(ibo_bound) =>
                            {
                                target.draw(vbo_bound, ibo_bound, &program, &uniform! { matrix: matrix },
                                    &Default::default()).unwrap();
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
    }
}

/// Contains all the shape information needed to draw an object, mainly surfaces and textures.
pub struct Mesh
{
    file: String,
    surfaces: Vec<Surface>,

    // TODO: Mesh' relative orientation to the parent object
    //transform: Mat4x4,
}

impl Mesh
{
    pub fn new() -> Mesh
    {
        Mesh {
            file: "".to_string(),
            surfaces: Vec::new(),
        }
    }

    pub fn load_from_file(self, _display: &glium::Display, _file: String) -> bool
    {
        // TODO: What file format do we want to support? Not sure what the common 3D format is today.
        // TODO: Handle passing to VBO/IBO
        true
    }

    // Creates a 2x2x2 cube at the origin, loads the vertex buffer and index buffer onto the GPU
    //
    // TODO: Shader should be stored with the mesh
    pub fn load_cube(mut self, display: &glium::Display) -> Self
    {
        implement_vertex!(VBOPos, position);
        let verts: [VBOPos; 24] = [
            // Top
            VBOPos{position: [-1.0,  1.0, -1.0]},    // 0
            VBOPos{position: [ 1.0,  1.0, -1.0]},    // 1
            VBOPos{position: [-1.0,  1.0,  1.0]},    // 2
            VBOPos{position: [ 1.0,  1.0,  1.0]},    // 3

            // Bottom]
            VBOPos{position: [-1.0, -1.0, -1.0]},   // 4
            VBOPos{position: [ 1.0, -1.0, -1.0]},   // 5
            VBOPos{position: [-1.0, -1.0,  1.0]},   // 6
            VBOPos{position: [ 1.0, -1.0,  1.0]},   // 7

            //Front]
            VBOPos{position: [-1.0,  1.0,  1.0]},   // 8
            VBOPos{position: [ 1.0,  1.0,  1.0]},   // 9
            VBOPos{position: [-1.0, -1.0,  1.0]},   // 10
            VBOPos{position: [ 1.0, -1.0,  1.0]},   // 11

            //Back]
            VBOPos{position: [-1.0,  1.0, -1.0]},  // 12
            VBOPos{position: [ 1.0,  1.0, -1.0]},  // 13
            VBOPos{position: [-1.0, -1.0, -1.0]},  // 14
            VBOPos{position: [ 1.0, -1.0, -1.0]},  // 15

            //Left]
            VBOPos{position: [-1.0,  1.0,  1.0]},  // 16
            VBOPos{position: [-1.0,  1.0, -1.0]},  // 17
            VBOPos{position: [-1.0, -1.0,  1.0]},  // 18
            VBOPos{position: [-1.0, -1.0, -1.0]},  // 19

            //Right]
            VBOPos{position: [ 1.0,  1.0,  1.0]},   // 20
            VBOPos{position: [ 1.0,  1.0, -1.0]},   // 21
            VBOPos{position: [ 1.0, -1.0,  1.0]},   // 22
            VBOPos{position: [ 1.0, -1.0, -1.0]}    // 23
        ];

        let indices: [u32; 36] = [
            //Top
            0, 1, 2,
            2, 3, 1,

            //Bottom
            4, 5, 6,
            6, 7, 5,

            //Front
            8, 9, 10,
            10, 11, 9,

            //Back
            12, 13, 14,
            14, 15, 13,

            //Left
            16, 17, 18,
            18, 19, 17,

            //Right
            20, 21, 22, 
            22, 23, 21
        ];

        let mut surface: Surface = Surface::new();

        surface.vbo = VertexBuffer::Pos{ vbo: Some(glium::VertexBuffer::new(display, &verts).unwrap())};
        surface.ibo = Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap());
        self.surfaces.push(surface);
        self
    }
    
    pub fn render(&self, target: &mut glium::Frame, program: &glium::Program)
    {
        for surface in self.surfaces.iter()
        {
            surface.render(target, program);
        }
    }
}