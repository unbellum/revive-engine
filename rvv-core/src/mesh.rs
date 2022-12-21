pub struct Mesh
{
    file: String,
    format: MeshFormat,
    gpu_id: u32,

    // TODO: Mesh' relative orientation to the parent object
    //transform: Mat4x4,
}

pub enum MeshFormat
{
    PosNorm,
    PosNormTex,
}

impl Mesh
{
    pub fn load_from_file(_file: String) -> bool
    {
        // TODO: What file format do we want to support? Not sure what the common 3D format is today.
        true
    }
}