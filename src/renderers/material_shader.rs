use glam::{Mat4, Vec3};

use crate::{
    buffers::{geometry::GeometryBuffer, uniform::UniformBuffer},
    camera::Camera, 
    geometries::{ColorElement, NormalElement, PositionElement, TexCoordElement},
    texture2d::Texture2d
};

use crate::renderers::material_buffer::MaterialBuffer;
use super::material_buffer::Material;

#[derive(Debug)]
pub struct MaterialShader{
    pub render_pipeline: wgpu::RenderPipeline,
    geometry_buffer : GeometryBuffer,

    model_matrix_buffer : UniformBuffer<Mat4>,
    vs_uniforms_bind_group : wgpu::BindGroup,

    material_buffer : MaterialBuffer,
    material_bind_group: wgpu::BindGroup,

    diffuse_bind_group: wgpu::BindGroup,

    rot_angle : f32,
}


impl MaterialShader {
    pub fn new( device: &wgpu::Device, 
                queue: &wgpu::Queue,
                format: wgpu::TextureFormat, 
                positions : &[PositionElement],
                normals : &[NormalElement],
                colors : &[ColorElement],
                tex_coords : &[TexCoordElement],
                indices : &[u32],
                camera : &Camera ) -> Self {

        
        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/MaterialShader.wgsl"));
        let geometry_buffer = GeometryBuffer::new(
            &device,
            positions,
            normals,
            colors,
            tex_coords,
            indices);

        //
        // SETUP VERTEX UNIFORMS
        // 
        let model_matrix_buffer = UniformBuffer::new(
            &device, 
            Mat4::IDENTITY,
            Some("Vertex model matrix uniform buffer"));

                
        let vs_uniforms_group_layout = 
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
                label :Some( "material vs uniforms layout group"),
            }
        );

        let vs_uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &vs_uniforms_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_matrix_buffer.buffer.as_entire_binding(),
                },
            ],
            label :Some( "material vs uniforms buffer group"),
        });

        //
        // MATERIAL BUFFER SETUP
        // 
        let mut material = Material::new(
            Vec3::new(0.5,0.2,0.1),
            Vec3::new(0.3,0.0,0.3),
            Vec3::new(0.5,0.2,0.1));
        material.diffuse_intensity = 2.0;

        let label = "Material shader material buffer";
        let material_buffer = MaterialBuffer::new(&device,material, label);

        let binding_index = 0;

        let material_group_layout = 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[MaterialBuffer::entry_layout(binding_index)],
                label :Some( "material uniform layout group"),
            }
        );

        let material_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &material_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: binding_index,
                    resource: material_buffer.binding_resource(),
                }
            ],
            label :Some( "material uniform buffer group"),
        });

        //
        // TEXTURE BINDING GROUP
        // 
        let texture_bytes = include_bytes!("../assets/test_texture.png");
        let texture = Texture2d::from_bytes(&device, &queue, texture_bytes, "test_texture").expect("Texture");

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });


        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(&texture.view),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&texture.sampler),
                    }
                ],
                label: Some("diffuse_bind_group"),
            }
        );

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[
                    &vs_uniforms_group_layout,      // bind group 0
                    &camera.bind_group_layout,      // bind group 1
                    &texture_bind_group_layout,     // bind group 2
                    &material_group_layout,    // bind group 3
                ],    
                push_constant_ranges: &[],
            }
        );


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render pipeline"),
            layout: Some( &render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "material_vs",
                buffers: &[
                    PositionElement::desc::<0>(),
                    ColorElement::desc::<1>(),
                    TexCoordElement::desc::<2>(),
                    NormalElement::desc::<3>() ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                //entry_point: "material_fs",
                entry_point: "material_flat_fs",
                targets: &[Some(wgpu::ColorTargetState {
                    format: format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            
            primitive: Default::default(), 

            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture2d::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),

            multisample: Default::default(),
            multiview: Default::default(),
        });

        Self {
            render_pipeline,
            geometry_buffer,

            model_matrix_buffer,
            vs_uniforms_bind_group,

            material_buffer,
            diffuse_bind_group,
            material_bind_group,

            rot_angle : 0.0,
        }
    }

    
    pub fn update(&mut self, queue: &wgpu::Queue) {
        self.rot_angle += 0.5;
        let trans = Mat4::from_translation(Vec3::new(0.0_f32, 0.0_f32, 3.0));
        let rot = Mat4::from_axis_angle(Vec3::X, self.rot_angle.to_radians() );
        let scale = Mat4::from_scale(Vec3::new(1.0,1.0,1.0));
        self.model_matrix_buffer.data =  trans * rot * scale;
        self.model_matrix_buffer.update(queue);
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a>, camera:&'a Camera ) {
        render_pass.set_pipeline(&self.render_pipeline); // setup renderpipeline
        render_pass.set_bind_group(0, &self.vs_uniforms_bind_group, &[]);
        camera.draw(render_pass, 1);
        //render_pass.set_bind_group(1, &self.camera.bind_groupdiffuse_bind_group, &[]);
        render_pass.set_bind_group(2, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(3, &self.material_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.geometry_buffer.position_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.geometry_buffer.color_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.geometry_buffer.texcoord_buffer.slice(..));

        if let Some(normal_buffer) = &self.geometry_buffer.normal_buffer {
            render_pass.set_vertex_buffer(3, normal_buffer.slice(..));
        }

        if let Some(index_buffer) = &self.geometry_buffer.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.geometry_buffer.num_indices, 0, 0..1);

        } else {
            render_pass.draw(0..self.geometry_buffer.num_vertices, 0..1); // draw 3 vertices with pipeline    
        }
    }

}
