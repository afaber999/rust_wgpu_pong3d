
#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    //tex_coords: [f32; 2], // CHANGED FOR TEXTURE
}

impl Vertex {   
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}    

// create array with position and color data
pub const QUAD: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], },    // 0
    Vertex { position: [ 0.5, -0.5, 0.0], },    // 1
    Vertex { position: [ 0.5,  0.5, 0.0], },    // 2
    //Vertex { position: [-0.5,  0.5, 0.0], },    // 3
];

// impl Quad {
//     pub fn new() -> Self {
//         Self {
//             positions : [
//                 -0.5,-0.5, 0.0,
//                 -0.5, 0.5, 0.0,
//                  0.5,-0.5, 0.0, ],
//         }
//     }
// }

// #[repr(C)]
// #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//     position: [f32; 3],
//     tex_coords: [f32; 2], // CHANGED FOR TEXTURE
// }

// // create array with position and color data
// #[allow(dead_code)]
// const QUAD1: &[Vertex] = &[
//     Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },    // 0   CHANGED COORDS FOR TEXTURE
//     Vertex { position: [ 0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },    // 1
//     Vertex { position: [ 0.5,  0.5, 0.0], tex_coords: [1.0, 0.0] },    // 2
//     Vertex { position: [-0.5,  0.5, 0.0], tex_coords: [0.0, 0.0] },    // 3
// ];