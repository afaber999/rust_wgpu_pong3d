use wgpu::util::DeviceExt;

pub trait UnformBufferData : bytemuck::Pod {
    fn raw_view<'a>(&'a self) -> &'a[u8] {
        bytemuck::bytes_of(self)
    }
}

#[derive(Debug)]
pub struct UniformBuffer<T: UnformBufferData> {
    pub data : T,
    pub buffer : wgpu::Buffer, 
}

impl<T: UnformBufferData> UniformBuffer<T> {
    pub fn new( device:&wgpu::Device, data : T, label: Option<&str>) -> Self 
    {
        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label,
                contents: data.raw_view(),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            }
        );

        Self {
            data,
            buffer,
        }
    }

    pub fn update(&self, queue : &wgpu::Queue) {
        queue.write_buffer(&self.buffer, 0, self.data.raw_view())
    }

}


