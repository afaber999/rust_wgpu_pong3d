use glam::{Mat4, Vec3, Vec4};
use crate::buffers::uniform::UniformBuffer;

#[derive(Debug)]
pub struct Camera {
    pub buffer : UniformBuffer<Mat4>,
    pub bind_group : wgpu::BindGroup,
    pub bind_group_layout : wgpu::BindGroupLayout,
}

impl Camera {

    pub fn new(
        device: &wgpu::Device,
        mat : Mat4,
        label : &str ) -> Self {

        //dbg!(mat);
        let buffer = UniformBuffer::new(
            &device, 
            mat, 
            Some( &format!( "Vertex uniform buffer for {label}") ));
                
        let bind_group_layout = 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },                    
                        count: None,                        
                    },
                ],
                label :Some( "Camera uniform group"),
            }
        );

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.buffer.as_entire_binding(),
                },
            ],
            label :Some( "Camera uniform buffer group"),
        });

        Self {
            buffer,
            bind_group,
            bind_group_layout,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, mat : Mat4) {
        self.buffer.data = mat;
        self.buffer.update(queue);
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a>, bind_group_nr : u32 ) {
        render_pass.set_bind_group(bind_group_nr, &self.bind_group, &[]);
    }


}