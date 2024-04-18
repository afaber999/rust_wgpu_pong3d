use glam::{Mat4, Vec3};
use crate::buffers::uniform::UniformBuffer;

#[derive(Debug)]
pub struct Camera {
    pub buffer : UniformBuffer<Mat4>,
    pub bind_group : wgpu::BindGroup,
    pub bind_group_layout : wgpu::BindGroupLayout,

    proj_mat : Mat4,
    pub eye : Vec3,
    pub center : Vec3,
    up : Vec3,
}

impl Camera {

    fn proj_view_mat(proj_mat : &Mat4, eye : &Vec3, center: &Vec3, up: &Vec3) -> Mat4 {
        let view_mat = Mat4::look_at_lh(*eye, *center, *up);
        dbg!(view_mat);
        proj_mat.mul_mat4(&view_mat)
    }

    pub fn new(
        device: &wgpu::Device,
        proj_mat : Mat4,
        eye : Vec3,
        center : Vec3,
        up : Vec3,
        label : &str ) -> Self {


        //dbg!(mat);
        let buffer = UniformBuffer::new(
            &device, 
            Self::proj_view_mat(&proj_mat, &eye, &center, &up), 
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
            proj_mat,
            eye,
            center,
            up,
        }
    }

    pub fn update_projection(&mut self, queue: &wgpu::Queue, proj_mat : Mat4) {
        self.proj_mat = proj_mat;
        self.buffer.data = Self::proj_view_mat(&proj_mat, &self.eye, &self.center, &self.up); 
        self.buffer.update(queue);
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a>, bind_group_nr : u32 ) {
        render_pass.set_bind_group(bind_group_nr, &self.bind_group, &[]);
    }


}