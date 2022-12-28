// For now only support basic RGBA textures

pub struct Texture
{
    file: String,
    //format: TextureFormat,
    gpu_id: u32,
    
    // TODO:
    //transparent: bool,
    //clamp: ClampFormat,
}

//pub enum TextureFormat
//{
//    RGBA(u32),
//}

impl Texture
{
    pub fn load_from_file(_display: &glium::Display, _file: String) -> bool
    {
        // TODO: What file format do we want to support? PNG, BMP, TIFF?
        true
    }
}