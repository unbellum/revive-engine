// For now only support basic RGBA textures

/// Handles the GPU resource for a texture used in rendering.
pub struct Texture
{
    file: String,
    //format: TextureFormat,
    //gpu_id: glium::texture::RawImage2d<u8>,
    
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
        // TODO: Glium uses the image package and we could probably do the same.
        true
    }

    pub fn load_checkerboard(self) -> Texture
    {
        //let mut image_data: [u8; 8*8*4] = [0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,
        //                                   1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,
        //                                   0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,
        //                                   1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,
        //                                   0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,
        //                                   1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,
        //                                   0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,
        //                                   1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,  1, 1, 1, 1,  0, 0, 0, 1,
        //];
        //self.gpu_id = glium::texture::RawImage2d::from_raw_rgba_reversed(&image_data, (8, 8));
        self
    }
}