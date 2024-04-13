use wgpu::util::DeviceExt;

use super::super::geometries::Vertex;

#[derive(Debug)]
pub struct GeometryBuffer {
    pub vertex_buffer : wgpu::Buffer,
    pub vertex_count : u32,
}

impl GeometryBuffer {
    pub fn new(device : &wgpu::Device, vertices : &[Vertex]) -> Self {

        // create vertex buffer
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        
        let vertex_count = vertices.len() as u32;

        Self {
            vertex_buffer,
            vertex_count,
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        Vertex::desc()
    }

}

