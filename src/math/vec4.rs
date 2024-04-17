use crate::buffers::uniform::UnformBufferData;


#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vec4{
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Vec4{
    pub fn new(x: f32, y: f32, z: f32, w: f32) ->Self{
        Self {
            x,
            y,
            z,
            w,
        }
    }
}


impl UnformBufferData for Vec4 {
    fn raw_view<'a>(&'a self) -> &'a[u8] {
        bytemuck::bytes_of(self)
    }
}