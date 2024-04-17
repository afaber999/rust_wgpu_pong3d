// use crate::buffers::uniform::UnformBufferData;

// #[repr(C)]
// #[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// pub struct Mat4x4{
//     pub data : [f32; 16],
// }

// impl Mat4x4{
//     pub fn new() ->Self{
//         Self { data : [
//             1.0, 0.0, 0.0, 0.0,
//             0.0, 1.0, 0.0, 0.0,
//             0.0, 0.0, 1.0, 0.0,
//             0.0, 0.0, 0.0, 1.0, ],
//         }
//     }


//     pub fn translate(&mut self, x:f32, y:f32, z:f32) {
//         self.data[3]=x;
//         self.data[7]=y;
//         self.data[11]=z;
//     }

// }

// impl UnformBufferData for Mat4x4 {}