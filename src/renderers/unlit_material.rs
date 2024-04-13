use crate::{buffers::geometry::GeometryBuffer, geometries::Vertex};

#[derive(Debug)]
pub struct UnlitMaterial{
    pub render_pipeline: wgpu::RenderPipeline,
    geometry_buffer : GeometryBuffer,
}

impl UnlitMaterial {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, vertices : &[Vertex], indices : &[u32]) -> Self {

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/UnlitMaterialShader.wgsl"));

        let geometry_buffer = GeometryBuffer::new(&device, vertices, indices);

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Unlit render pipeline"),
            layout: Default::default(),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "unlit_material_vs",
                buffers: &[GeometryBuffer::vertex_buffer_desc()],
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
        }
    }

    pub fn draw<'a>( &'a self, render_pass: &mut wgpu::RenderPass<'a> ) {
        render_pass.set_pipeline(&self.render_pipeline); // setup renderpipeline
        render_pass.set_vertex_buffer(0, self.geometry_buffer.vertex_buffer.slice(..));

        if let Some(index_buffer) = &self.geometry_buffer.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.geometry_buffer.index_count, 0, 0..1);

        } else {
            render_pass.draw(0..self.geometry_buffer.vertex_count, 0..1); // draw 3 vertices with pipeline    
        }
    }

}
