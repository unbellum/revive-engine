pub struct Mesh
{
    file: String,
    format: MeshFormat,
    gpu_id: u32,
}

pub enum MeshFormat
{
    PosNorm,
    PosNormTex,
}