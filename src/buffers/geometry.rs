use wgpu::util::DeviceExt;
use crate::geometries::{ColorElement, NormalElement, PositionElement, TexCoordElement};
//use super::super::geometries::Vertex;

#[derive(Debug)]
pub struct GeometryBuffer {
    pub position_buffer : wgpu::Buffer,
    pub color_buffer : wgpu::Buffer,
    pub texcoord_buffer : wgpu::Buffer,
    pub index_buffer : Option<wgpu::Buffer>,
    pub normal_buffer : Option<wgpu::Buffer>,
    pub num_vertices : u32,
    pub num_indices : u32,

}

impl GeometryBuffer {
    pub fn new(device : &wgpu::Device,
         positions : &[PositionElement], 
         normals : &[NormalElement], 
         colors : &[ColorElement],
         tex_coords : &[TexCoordElement],
         indices : &[u32] ) -> Self {

        // create vertex buffer for positions
        let position_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Position Buffer"),
                contents: bytemuck::cast_slice(positions),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
        let num_vertices = positions.len() as u32;

        let normal_buffer = if normals.len() > 0 {
            // create index buffer
            Some( device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
                    label: Some("Normals Buffer"),
                    contents: bytemuck::cast_slice(normals),
                    usage: wgpu::BufferUsages::VERTEX,
                }))
        } else {
            None
        };

        // create vertex buffer for colors
        let color_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Color Buffer"),
                contents: bytemuck::cast_slice(colors),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            }
        );

        // create vertex buffer for tex_coords
        let texcoord_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("TexCoord Buffer"),
                contents: bytemuck::cast_slice(tex_coords),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );
      
        let index_buffer = if indices.len() > 0 {
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




        let num_indices = indices.len() as u32;

        Self {
            position_buffer,
            normal_buffer,
            texcoord_buffer,
            color_buffer,
            index_buffer,
            num_vertices,
            num_indices,
        }
    }

    // pub fn descriptors() -> &'static[ wgpu::VertexBufferLayout<'static> ] {
    //     &[PositionElement::desc(), ColorElement::desc()]
    // }
}

