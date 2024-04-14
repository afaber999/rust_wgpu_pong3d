#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PositionElement {
    position: [f32; 3],
}
impl PositionElement {   
    pub fn desc(location: usize) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
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

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorElement {
    color: [f32; 4],
}
impl ColorElement {   
    pub fn desc(location: usize) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ]
        }

    }
}  

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TexCoordElement {
    position: [f32; 4],
}

impl TexCoordElement {   
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ]
        }

    }
}  

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
    //tex_coords: [f32; 2], // CHANGED FOR TEXTURE
}

impl Vertex {   
    pub fn desc( has_color: bool) -> wgpu::VertexBufferLayout<'static> {
        if has_color {
            wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[
                    wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x3,
                    },
                    wgpu::VertexAttribute {
                        offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                        shader_location: 1,
                        format: wgpu::VertexFormat::Float32x4,
                    },
                ]
            }    
        } else {
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

// create array with position and position data
pub const QUAD_VERTICES: &[Vertex] = &[
    Vertex { position: [-0.5, -0.5, 0.0],color: [1.0, 0.0, 0.0, 1.0] },    // bottom left
    Vertex { position: [-0.5,  0.5, 0.0],color: [0.0, 1.0, 0.0, 1.0] },    // top left
    Vertex { position: [ 0.5, -0.5, 0.0],color: [0.0, 0.0, 1.0, 1.0] },    // bottom right
    Vertex { position: [ 0.5,  0.5, 0.0],color: [1.0, 1.0, 0.0, 1.0] },    // top right
];

pub const QUAD_INDICES: &[u32] = &[
    0, 1, 2,
    1, 3, 2,
];



pub struct QuadGeometry<'a> {
    pub positions : &'a[PositionElement],
    pub colors : &'a[ColorElement],
    pub indices : &'a[u32],
}

impl<'a> QuadGeometry<'a> {
    pub fn new() -> Self {
        Self {
            positions : &QUAD_POSITIONS,
            colors : &QUAD_COLORS,
            indices : &QUAD_INDICES,
        }
    }
}
