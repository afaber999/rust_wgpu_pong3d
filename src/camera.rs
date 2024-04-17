use glam::{Mat4, Vec3, Vec4};
use crate::buffers::uniform::UniformBuffer;

#[derive(Debug)]
pub struct Camera {
    pub buffer : UniformBuffer<Mat4>,
    pub bind_group : wgpu::BindGroup,
    pub bind_group_layout : wgpu::BindGroupLayout,
}

impl Camera {
    pub fn new_orthographic(
        device: &wgpu::Device,
        left : f32,
        right : f32,
        bottom : f32,
        top : f32,
        near : f32,
        far : f32) -> Self {

        let mat = Mat4::orthographic_lh(left, right, bottom, top, near, far); 
        //dbg!(mat);   

        let buffer = UniformBuffer::new(
            &device, 
            mat, 
            Some("Vertex model matrix uniform buffer"));

                
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

    pub fn update(&mut self, queue: &wgpu::Queue) {
        // let trans = Mat4::from_translation(Vec3::new(0.3, 0.5, 0.0));
        // let rot = Mat4::from_axis_angle(Vec3::AXES[2], 20.0_f32.to_degrees() );
        // let scale = Mat4::from_scale(Vec3::new(2.0,0.4,1.0));
        // self.buffer.data =  trans * rot * scale;
        // self.buffer.update(queue);
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a>, bind_group_nr : u32 ) {
        render_pass.set_bind_group(bind_group_nr, &self.bind_group, &[]);
    }


}