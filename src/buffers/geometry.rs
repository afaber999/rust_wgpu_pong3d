use wgpu::util::DeviceExt;

use super::super::geometries::Vertex;

#[derive(Debug)]
pub struct GeometryBuffer {
    pub vertex_buffer : wgpu::Buffer,
    pub index_buffer : Option<wgpu::Buffer>,
    pub vertex_count : u32,
    pub index_count : u32,
}

impl GeometryBuffer {
    pub fn new(device : &wgpu::Device, vertices : &[Vertex], indices : &[u32]) -> Self {
        // create vertex buffer
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let vertex_count = vertices.len() as u32;
        
        let mut index_buffer = if indices.len() > 0 {
            // create index buffer
            Some( device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(indices),
                    usage: wgpu::BufferUsages::INDEX,
                }))
        } else {
            None
        };

        let index_count = indices.len() as u32;

        Self {
            vertex_buffer,
            index_buffer,
            vertex_count,
            index_count,
        }
    }

    pub fn vertex_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        Vertex::desc()
    }

}

