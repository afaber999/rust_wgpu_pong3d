use glam::{Mat4, Vec2, Vec3, Vec4};

use crate::{buffers::{geometry::GeometryBuffer, uniform::{UniformBufferData, UniformBuffer}}, geometries::{ColorElement, PositionElement, TexCoordElement}, texture2d::Texture2d};


#[derive(Debug)]
pub struct UnlitMaterial{
    pub render_pipeline: wgpu::RenderPipeline,
    geometry_buffer : GeometryBuffer,

    texture_tiling_buffer : UniformBuffer<Vec2>,
    model_matrix_buffer : UniformBuffer<Mat4>,
    vs_uniforms_bind_group : wgpu::BindGroup,

    diffuse_color_buffer : UniformBuffer<Vec4>,
    diffuse_bind_group: wgpu::BindGroup,
    diffuse_color_bind_group: wgpu::BindGroup,
}


impl UnlitMaterial {
    pub fn new( device: &wgpu::Device, 
                queue: &wgpu::Queue,
                format: wgpu::TextureFormat, 
                positions : &[PositionElement],
                colors : &[ColorElement],
                tex_coords : &[TexCoordElement],
                indices : &[u32]) -> Self {



        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/UnlitMaterialShader.wgsl"));
        let geometry_buffer = GeometryBuffer::new(&device, positions, colors, tex_coords, indices);

        //
        // SETUP VERTEX UNIFORMS
        // 
        let texture_tiling_buffer = UniformBuffer::new(
            &device, 
            Vec2::new(1.0,3.0), 
            Some("Vertex texture tiling uniform buffer"));

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
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },                    
                        count: None,                        
                    },
                ],
                label :Some( "Unlit material vs uniforms layout group"),
            }
        );

        let vs_uniforms_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &vs_uniforms_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_matrix_buffer.buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: texture_tiling_buffer.buffer.as_entire_binding(),
                },
            ],
            label :Some( "Unlit material vs uniforms buffer group"),
        });


        //
        // DIFFUSE COLOR UNIFORM
        // 
        let diffuse_color_buffer = UniformBuffer::new(
            &device, 
            Vec4::new(1.0,0.0, 0.0,1.0), 
            Some("diffuse color buffer"));


        let diffuse_color_group_layout = 
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },                    
                        count: None,                        
                    }
                ],
                label :Some( "Unlit material diffuse color uniform layout group"),
            }
        );

        let diffuse_color_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &diffuse_color_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: diffuse_color_buffer.buffer.as_entire_binding(),
                }
            ],
            label :Some( "Unlit material diffuse color uniform buffer group"),
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
                    &texture_bind_group_layout,     // bind group 1
                    &diffuse_color_group_layout,    // bind group 2
                ],    
                push_constant_ranges: &[],
            }
        );


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Unlit render pipeline"),
            layout: Some( &render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "unlit_material_vs",
                buffers: &[
                    PositionElement::desc::<0>(),
                    ColorElement::desc::<1>(),
                    TexCoordElement::desc::<2>() ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "unlit_material_fs",
                targets: &[Some(wgpu::ColorTargetState {
                    format: format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: Default::default(), 
            depth_stencil: Default::default(),
            multisample: Default::default(),
            multiview: Default::default(),
        });

        Self {
            render_pipeline,
            geometry_buffer,

            texture_tiling_buffer,
            model_matrix_buffer,
            vs_uniforms_bind_group,

            diffuse_color_buffer,
            diffuse_bind_group,
            diffuse_color_bind_group,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue) {
        //self.textiling_buffer.data.x *= 1.01;
        //self.textiling_buffer.update(queue);

        //self.model_matrix_buffer.data.translate(0.5, 1.0, 0.0);
        let trans = Mat4::from_translation(Vec3::new(0.3, 0.5, 0.0));
        let rot = Mat4::from_axis_angle(Vec3::AXES[2], 20.0_f32.to_degrees() );
        let scale = Mat4::from_scale(Vec3::new(2.0,0.4,1.0));
        self.model_matrix_buffer.data =  trans * rot * scale;

        self.model_matrix_buffer.update(queue);
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a> ) {
        render_pass.set_pipeline(&self.render_pipeline); // setup renderpipeline
        render_pass.set_bind_group(0, &self.vs_uniforms_bind_group, &[]);
        render_pass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        render_pass.set_bind_group(2, &self.diffuse_color_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.geometry_buffer.position_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.geometry_buffer.color_buffer.slice(..));
        render_pass.set_vertex_buffer(2, self.geometry_buffer.texcoord_buffer.slice(..));

        if let Some(index_buffer) = &self.geometry_buffer.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.geometry_buffer.num_indices, 0, 0..1);

        } else {
            render_pass.draw(0..self.geometry_buffer.num_vertices, 0..1); // draw 3 vertices with pipeline    
        }

    }

}
