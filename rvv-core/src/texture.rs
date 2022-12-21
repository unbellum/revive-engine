// For now only support basic RGBA textures

pub struct Texture
{
    file: String,
    //format: TextureFormat,
    gpu_id: u32
}

//pub enum TextureFormat
//{
//    RGBA(u32),
//}

pub fn load_texture(file: String) -> Texture
{
    // Load from file
    Texture{file: file, /*format: RGBA,*/ gpu_id: 0}
}