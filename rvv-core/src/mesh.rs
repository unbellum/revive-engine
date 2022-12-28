
// TODO: Need to learn some generics to move forward here. Want a general Mesh object
// but it needs to know what type of vertices it will use at run time. I expect that
// the best way to do this is to define the Mesh type when the mesh object is created.
//
// Workflow: User creates a new world object in the scene which defaults to the cube
// mesh in the editor. This cube mesh will only contain position information for now.
// Once the user decides which mesh to replace the cube with they can then load a
// new mesh from file.
//
// This raises the problem that the WorldObject stores a vector of Mesh objects and
// these must all have matching types. Having the Mesh object be a generic interface
// also does not work in the same way that you cannot store Vec<Vec<Unknowtype> >. You
// must know when the object is declared what will go into it.
//
// Then how do we solve this? A new type/interface per mesh style? This would then
// necessitate a new vector for each type in the WorldObject which feel clunky.
//
// It looks like enums are decently powerful in rust. May need to learn about them too.
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
pub struct Mesh
{
    file: String,
    vbo: VertexBuffer,
    ibo: Option<glium::IndexBuffer<u32> >,

    // TODO: Mesh' relative orientation to the parent object
    //transform: Mat4x4,
}

impl Mesh
{
    pub fn new() -> Mesh
    {
        Mesh {
            file: "".to_string(),
            vbo: VertexBuffer::Pos { vbo: None },
            ibo: None
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

        self.vbo = VertexBuffer::Pos{ vbo: Some(glium::VertexBuffer::new(display, &verts).unwrap())};
        self.ibo = Some(glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &indices).unwrap());
        self
    }
}