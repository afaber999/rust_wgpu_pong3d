#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PositionElement {
    position: [f32; 3],
}
impl PositionElement {   
    pub fn desc<const N: u32>() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: N,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }

    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorElement {
    color: [f32; 4],
}
impl ColorElement {   
    pub fn desc<const N: u32>() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: N,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }

    }
}  

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexCoordElement {
    position: [f32; 2],
}

impl TexCoordElement {   
    pub fn desc<const N: u32>() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: N,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }

    }
}  

// create array with position and position data
pub const QUAD_POSITIONS: &[PositionElement] = &[
    PositionElement { position: [-0.5, -0.5, 0.0] },    // bottom left
    PositionElement { position: [-0.5,  0.5, 0.0] },    // top left
    PositionElement { position: [ 0.5, -0.5, 0.0] },    // bottom right
    PositionElement { position: [ 0.5,  0.5, 0.0] },    // top right
];

pub const QUAD_COLORS: &[ColorElement] = &[
    ColorElement { color: [1.0, 0.0, 0.0, 1.0] },    // bottom left
    ColorElement { color: [0.0, 1.0, 0.0, 1.0] },    // top left
    ColorElement { color: [0.0, 0.0, 1.0, 1.0] },    // bottom right
    ColorElement { color: [1.0, 1.0, 0.0, 1.0] },    // top right
];

pub const QUAD_TEXCOORDS: &[TexCoordElement] = &[
    TexCoordElement { position: [0.0, 0.0] },    // bottom left
    TexCoordElement { position: [0.0, 1.0] },    // top left
    TexCoordElement { position: [1.0, 0.0] },    // bottom right
    TexCoordElement { position: [1.0, 1.0] },    // top right
];


pub const QUAD_INDICES: &[u32] = &[
    0, 1, 2,
    1, 3, 2,
];


pub struct QuadGeometry<'a> {
    pub positions : &'a[PositionElement],
    pub colors : &'a[ColorElement],
    pub tex_coords : &'a[TexCoordElement],
    pub indices : &'a[u32],
}

impl<'a> QuadGeometry<'a> {
    pub fn new() -> Self {
        Self {
            positions : &QUAD_POSITIONS,
            colors : &QUAD_COLORS,
            indices : &QUAD_INDICES,
            tex_coords : &QUAD_TEXCOORDS,
        }
    }
}
