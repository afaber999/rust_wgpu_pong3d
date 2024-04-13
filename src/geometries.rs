
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
pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0], },    // bottom left
    Vertex { position: [-0.5,  0.5, 0.0], },    // top left
    Vertex { position: [ 0.5, -0.5, 0.0], },    // bottom right
    Vertex { position: [ 0.5,  0.5, 0.0], },    // top right
];

pub const QUAD_INDICES: &[u32] = &[
    0, 1, 2,
    1, 3, 2,
];


pub struct QuadGeometry<'a> {
    pub vertices : &'a[Vertex],
    pub indices : &'a[u32],
}

impl<'a> QuadGeometry<'a> {
    pub fn new() -> Self {
        Self {
            vertices : &QUAD_VERTICES,
            indices : &QUAD_INDICES,
        }
    }
}
