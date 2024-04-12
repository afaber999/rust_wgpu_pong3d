
pub struct UnlitMaterial{
    pub render_pipeline: wgpu::RenderPipeline,
}

impl UnlitMaterial {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {

        let shader = device.create_shader_module(wgpu::include_wgsl!("./shaders/UnlitMaterialShader.wgsl"));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Unlit render pipeline"),
            layout: Default::default(),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "unlit_material_vs",
                buffers: &[],
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
        }
    }


}
