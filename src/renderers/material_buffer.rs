
use glam::Vec3;
use crate::buffers::uniform::{UniformBuffer, UniformBufferData};


#[repr(C)]
#[derive(Default, Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Material {
    pub ambient_color :Vec3,
    // Due to uniforms requiring 16 byte (4 float) spacing, we need to use a padding field here
    _padding0 : u32,
    pub diffuse_color :Vec3,
    _padding1 : u32,
    pub specular_color :Vec3,
    _padding2 : u32,
}


impl Material {
    pub fn new(ambient_color  : Vec3, diffuse_color : Vec3, specular_color : Vec3) -> Self {
        Self {
            ambient_color,
            diffuse_color,
            specular_color,
            _padding0: 0, 
            _padding1: 0,
            _padding2: 0,
        }
    }
}


impl UniformBufferData for Material {}

#[derive(Debug)]
pub struct MaterialBuffer {
    buffer : UniformBuffer<Material>,
}

impl MaterialBuffer {
    pub fn new(device: &wgpu::Device, material: Material, label: &str ) -> Self {
        let label = Some(label);
        let data = material;
        Self { buffer : UniformBuffer::new(&device,data,label ),}
    }

    pub fn binding_resource(&self) ->wgpu::BindingResource {
        self.buffer.buffer.as_entire_binding()
    }

    pub fn update(&self, queue: &wgpu::Queue) {
        self.buffer.update(queue);
    }

    pub fn entry_layout(binding_index : u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: binding_index,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },                    
            count: None,                        
        }        
    }    
}
